use anyhow::Context;
use byteorder::{LittleEndian, ReadBytesExt};
use std::collections::HashMap;
use std::fmt::Debug;
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use log::error;
use thiserror::Error;
use try_insert_ext::EntryInsertExt;
use crate::helpers::PythonVersion;

mod helpers;

#[derive(Error, Debug)]
pub enum MPKError {
    #[error("mpkinfo header is invalid")]
    InvalidInfoHeader(),
    #[error("error reading")]
    ReadError(#[from] std::io::Error),
}

#[derive(Debug)]
pub struct MPKFileEntryV1 {
    name: String,
    offset: u32,
    size: u32,
    is_folder: bool,
    file_number: u32,
}

#[derive(Debug)]
pub struct MPKFileEntryV2 {
    name: [u8; 3],
    offset: u32,
    size: u32,
    flags: u32,
    hash: u32,
    file_number: u32,
}

#[derive(Debug)]
enum MPKFileEntry {
    V1(MPKFileEntryV1),
    V2(MPKFileEntryV2),
}

impl MPKFileEntry {
    fn is_folder(&self) -> bool {
        match self {
            MPKFileEntry::V1(file) => file.is_folder,
            MPKFileEntry::V2(file) => file.flags & 1 == 1,
        }
    }

    fn file_number(&self) -> u32 {
        match self {
            MPKFileEntry::V1(file) => file.file_number,
            MPKFileEntry::V2(file) => file.file_number,
        }
    }

    fn size(&self) -> u32 {
        match self {
            MPKFileEntry::V1(file) => file.size,
            MPKFileEntry::V2(file) => file.size,
        }
    }

    fn offset(&self) -> u32 {
        match self {
            MPKFileEntry::V1(file) => file.offset,
            MPKFileEntry::V2(file) => file.offset,
        }
    }

    fn name(&self) -> String {
        match self {
            MPKFileEntry::V1(file) => file.name.to_string(),
            MPKFileEntry::V2(file) => if self.is_folder() {
                format!("{}", std::str::from_utf8(&file.name).unwrap().replace("/", "_"))
            } else {
                format!("file_{}_{}.{}", file.file_number, file.hash, std::str::from_utf8(&file.name).unwrap())
            }
        }
    }
}

#[derive(Debug)]
pub struct MPKFileHeader {
    version: u32,
    file_count: u32,
}

impl MPKFileHeader {
    fn read_header<T>(reader: &mut std::io::BufReader<T>) -> Result<MPKFileHeader, MPKError>
    where
        T: std::io::Read,
        T: std::io::Seek,
    {
        let version = reader.read_u32::<LittleEndian>()?;
        let file_count = reader.read_u32::<LittleEndian>()?;
        Ok(Self {
            version,
            file_count,
        })
    }
}

#[derive(Debug)]
pub struct MPKFileReader {
    path: std::path::PathBuf,
    _file: std::fs::File,
    _header: MPKFileHeader,
    files: Vec<MPKFileEntry>,
}

impl MPKFileReader {
    pub fn new<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<Self> {
        let file = std::fs::File::open(&path).with_context(|| {
            format!(
                "Failed to read .mpkinfo file from {}",
                path.as_ref().to_string_lossy()
            )
        })?;
        let mut reader = BufReader::new(&file);
        let header = MPKFileHeader::read_header(&mut reader)?;

        let mut files = Vec::new();
        for _ in 0..header.file_count {
            // Read indices
            match header.version {
                1 => {
                    let name_length = reader.read_u16::<LittleEndian>()?;
                    let mut name_buffer = vec![0; name_length as usize];
                    reader.read_exact(&mut name_buffer)?;

                    let offset = reader.read_u32::<LittleEndian>()?;
                    let size = reader.read_u32::<LittleEndian>()?;
                    let flags = reader.read_u32::<LittleEndian>()?;

                    let is_folder = flags & 1 == 1;
                    let file_number = flags >> 1;

                    if is_folder {
                        assert!(size == 0);
                    }

                    files.push(MPKFileEntry::V1(MPKFileEntryV1 {
                        name: String::from_utf8(name_buffer)?,
                        offset,
                        size,
                        is_folder,
                        file_number,
                    }))
                }
                2 => {
                    let size = reader.read_u32::<LittleEndian>()?;
                    let flags = reader.read_u32::<LittleEndian>()?;
                    let t = reader.read_u8()?;
                    let mut type_buffer: [u8; 3] = [0; 3];
                    reader.read_exact(&mut type_buffer)?;
                    let hash = reader.read_u32::<LittleEndian>()?;
                    let offset = reader.read_u32::<LittleEndian>()?;
                    let file_number = flags >> 1;

                    if flags & 1 == 1 {
                        assert!(size == 0);
                    }

                    files.push(MPKFileEntry::V2(MPKFileEntryV2 {
                        name: type_buffer,
                        offset,
                        size,
                        flags,
                        hash,
                        file_number
                    }))
                }
                _ => return Err(MPKError::InvalidInfoHeader().into()),
            }
        }

        Ok(MPKFileReader {
            path: path.as_ref().as_os_str().to_os_string().into(),
            _file: file,
            _header: header,
            files,
        })
    }

    pub fn extract_files<P: AsRef<std::path::Path>>(&self, out_dir: P) -> anyhow::Result<()> {
        use indicatif::ProgressBar;

        let bar = ProgressBar::new(self.files.len() as u64);

        std::fs::create_dir_all(&out_dir)?;

        let basename = self.path.file_stem().unwrap().to_str().unwrap();
        let parent_path = self.path.parent().unwrap();

        let mut mpk_map = HashMap::new();
        for file in &self.files {
            if !file.is_folder() {
                let file_number = file.file_number();
                let mpk_file = mpk_map.entry(file_number).or_try_insert_with(|| {
                    let mpk_file = if file_number == 0 {
                        format!("{}.mpk", basename)
                    } else {
                        format!("{}{}.mpk", basename, file_number)
                    };
                    std::fs::File::open(parent_path.join(mpk_file))
                })?;

                let offset = mpk_file.seek(SeekFrom::Start(file.offset().into()))?;
                assert_eq!(offset, file.offset().into());
                let mut file_buffer = vec![0; file.size() as usize];
                mpk_file.read_exact(&mut file_buffer)?;

                let mut reader = std::io::Cursor::new(&file_buffer);
                let mut magic = vec![0; 4];
                let _ = reader.read_exact(&mut magic);

                // TODO(alexander): This _should_ probably be optional
                let (file_buffer, alt_file_name): (_, Option<String>) = if magic == b"ZZZ4" {
                    (
                        lz4_flex::decompress_size_prepended(&file_buffer[4..])?,
                        None,
                    )
                } else if magic == b"CCCC" {
                    let mut magic = vec![0; 4];
                    let _ = reader.read_exact(&mut magic);
                    if magic == b"ZZZ4" {
                        let uncompressed_size = reader.read_i32::<LittleEndian>()?;
                        let mut buffer = vec![0; 0_usize];
                        reader.read_to_end(&mut buffer)?;
                        // There is an unknown "overhang" of 20 bytes at the end, no idea what it is
                        // Ignore for now
                        // _could_ be a sha1 actually
                        (
                            lz4_flex::decompress(
                                &buffer[..buffer.len() - 20],
                                uncompressed_size as usize,
                            )?,
                            None,
                        )
                    } else if magic == b"LZMA" {
                        let uncompressed_size = reader.read_i32::<LittleEndian>()?;
                        let mut buffer = vec![0; 0_usize];
                        reader.read_to_end(&mut buffer)?;
                        let mut decompressed = vec![];
                        lzma_rs::lzma_decompress_with_options(
                            &mut std::io::Cursor::new(&buffer),
                            &mut decompressed,
                            &lzma_rs::decompress::Options {
                                unpacked_size: lzma_rs::decompress::UnpackedSize::UseProvided(
                                    Some(uncompressed_size as u64),
                                ),
                                memlimit: None,
                                allow_incomplete: false,
                            },
                        )?;
                        let file_name = Self::detect_file_name_with_extension(file, &decompressed);
                        (decompressed, Some(file_name))
                    } else {
                        // let file_name = Self::detect_file_name_with_extension(file, &file_buffer);
                        (file_buffer, None)
                    }
                } else if &magic[..2] == b"\xE2\x06" {
                    // This is a mangled zlib compressed file
                    // TODO(alexander): Move handling of these to a new crate
                    // TODO(alexander): Reduce number of vec allocations
                    reader.seek(SeekFrom::Start(0))?;
                    let mut buffer = vec![0; 0_usize];
                    reader.read_to_end(&mut buffer)?;

                    let offset = (buffer.len() - 8) % 37;
                    let end = 128 - offset;
                    let end = end.min(buffer.len());
                    // eprintln!("{} {} {}", buffer.len(), offset, end);
                    let head = &mut buffer[..end];
                    for x in head.iter_mut() {
                        *x ^= 154;
                    }
                    let end = if end == buffer.len() {
                        end
                    } else {
                        buffer.len() - 8
                    };
                    use compress::zlib;
                    let mut decoder = zlib::Decoder::new(&buffer[..end]);
                    let mut result_buffer = vec![];
                    decoder.read_to_end(&mut result_buffer)?;

                    if let Ok(file_name) = helpers::file_name_from_py_buffer(&result_buffer) {
                        if file_name.is_empty() {
                            let file_name = Self::detect_file_name_with_extension(file, &result_buffer);
                            (result_buffer, Some(file_name))
                        } else {
                            (
                                result_buffer,
                                Some(format!("Script/Python/{}c", file_name)),
                            )
                        }
                    } else {
                        let file_name = Self::detect_file_name_with_extension(file, &file_buffer);
                        (file_buffer, Some(file_name))
                    }
                } else {
                    (file_buffer, None)
                };

                let file_name = if let Some(file_name) = alt_file_name {
                    file_name
                } else {
                    file.name()
                };
                let out_file_path = out_dir.as_ref().join(file_name);
                std::fs::create_dir_all(out_file_path.parent().unwrap())?;

                let mut out_file = std::fs::File::create(&out_file_path)
                    .context(out_file_path.display().to_string())?;
                out_file.write_all(&file_buffer)?;
            } else {
                let out_file_path = out_dir.as_ref().join(file.name());
                std::fs::create_dir_all(out_file_path)?;
            }

            bar.inc(1);
        }

        bar.finish();

        Ok(())
    }

    fn detect_file_name_with_extension(file: &MPKFileEntry, decompressed: &Vec<u8>) -> String {
        let result = tree_magic_mini::from_u8(&decompressed);
        let extension = match result {
            "application/x-executable" => "exe",
            "application/x-cpio" => "cpio",
            "image/ktx" => "ktx",
            "image/png" => "png",
            "image/x-dds" => "dds",
            "image/x-win-bitmap" => "bmp",
            "application/xml" => "xml",
            "text/x-matlab" => "mat", // Maybe m instead?
            "application/x-apple-systemprofiler+xml" => "xml",
            "text/x-modelica" => "mo",
            "text/x-csrc" => "c",
            "font/ttf" => "ttf",
            "image/bmp" => "bmp",
            "application/zip" => "zip",
            "image/jpeg" => "jpg",
            "image/vnd.zbrush.pcx" => "pcx",
            "audio/mpeg" => "mp3",
            "audio/x-wav" => "wav",
            "audio/vnd.wave" => "wav",
            "application/x-java-jce-keystore" => "pem",
            "application/x-font-ttf" => "ttf",
            "application/octet-stream" => return file.name().to_owned(),
            "video/mp4" => "mp4",
            "text/plain" => "txt",
            _ => {
                error!("Unhandled mime type {}", result);
                "dat"
            }
        };
        let name = file.name();
        let path = std::path::Path::new(&name);
        if path.parent().unwrap().to_string_lossy().is_empty() {
            let file_name = format!("{}.{}", path.file_stem().unwrap().to_string_lossy(), extension);
            file_name
        } else {
            let file_name = format!("{}/{}.{}", path.parent().unwrap().to_string_lossy(), path.file_stem().unwrap().to_string_lossy(), extension);
            file_name
        }

    }
}
