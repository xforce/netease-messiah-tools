use anyhow::Context;
use byteorder::{LittleEndian, ReadBytesExt};
use std::collections::HashMap;
use std::fmt::Debug;
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use thiserror::Error;
use try_insert_ext::EntryInsertExt;

#[derive(Error, Debug)]
pub enum MPKError {
    #[error("mpkinfo header is invalid")]
    InvalidInfoHeader(),
    #[error("error reading")]
    ReadError(#[from] std::io::Error),
}

#[derive(Debug)]
pub struct MPKFileEntry {
    name: String,
    offset: u32,
    size: u32,
    is_folder: bool,
    file_number: u32,
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
        assert!(version == 1);
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
    pub fn new<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<Self>
    where
        P: Debug,
    {
        let file = std::fs::File::open(&path)
            .with_context(|| format!("Failed to read .mpkinfo file from {:?}", path))?;
        let mut reader = BufReader::new(&file);
        let header = MPKFileHeader::read_header(&mut reader)?;

        assert!(
            header.version == 1,
            "Currently only MPK Version 1 is supported"
        );

        // Read indices
        let mut files = Vec::new();
        for _ in 0..header.file_count {
            let name_length = reader.read_u16::<LittleEndian>()?;
            let mut name_buffer = vec![0; name_length as usize];
            reader.read_exact(&mut name_buffer)?;

            let offset = reader.read_u32::<LittleEndian>()?;
            let size = reader.read_u32::<LittleEndian>()?;
            let flags = reader.read_u32::<LittleEndian>()?;

            let is_folder = flags & 1 == 1;
            let file_number = flags >> 1;

            files.push(MPKFileEntry {
                name: String::from_utf8(name_buffer)?,
                offset,
                size,
                is_folder,
                file_number,
            })
        }

        Ok(MPKFileReader {
            path: path.as_ref().as_os_str().to_os_string().into(),
            _file: file,
            _header: header,
            files,
        })
    }

    // TODO(alexander): Add interface to operate on files

    fn read_object(reader: &mut std::io::Cursor<&Vec<u8>>) -> anyhow::Result<Vec<u8>> {
        let t = reader.read_u8()?;
        if t == 115 {
            let l = reader.read_u32::<LittleEndian>()?;
            let mut buf = vec![0; l as usize];
            reader.read_exact(&mut buf)?;
            Ok(buf)
        } else if t == 'y' as u8 {
            let l = reader.read_u64::<LittleEndian>()?;
            let l = reader.read_u64::<LittleEndian>()?;
            Ok(vec![])
        } else if t == 'l' as u8 {
            let m_size = reader.read_i32::<LittleEndian>()?;
            let actualSize = if m_size >= 0 { m_size } else { -m_size };
            for _ in 0..actualSize {
                reader.read_u16::<LittleEndian>()?;
            }
            Ok(vec![])
        } else if t == 'u' as u8 {
            let l = reader.read_u32::<LittleEndian>()?;
            let mut buf = vec![0; l as usize];
            reader.read_exact(&mut buf)?;
            Ok(buf)
        } else if t == 't' as u8 {
            let l = reader.read_u32::<LittleEndian>()?;
            let mut buf = vec![0; l as usize];
            reader.read_exact(&mut buf)?;
            Ok(buf)

            // TYPE_TUPLE
        } else if t == '(' as u8 {
            //
            let l = reader.read_u32::<LittleEndian>()?;
            for _ in 0..l {
                MPKFileReader::read_object(reader)?;
            }

            Ok(vec![])
        } else if t == 'g' as u8 {
            let l = reader.read_u64::<LittleEndian>()?;
            Ok(vec![])
        } else if t == 'N' as u8 {
            Ok(vec![])
        } else if t == 'i' as u8 || t == 'R' as u8 {
            let _ = reader.read_u32::<LittleEndian>()?;
            Ok(vec![])
        } else if t == 99 {
            reader.read_u32::<LittleEndian>()?;
            reader.read_u32::<LittleEndian>()?;
            reader.read_u32::<LittleEndian>()?;
            reader.read_u32::<LittleEndian>()?;

            MPKFileReader::read_object(reader)?; // code
            MPKFileReader::read_object(reader)?; // consts
            MPKFileReader::read_object(reader)?; // names
            MPKFileReader::read_object(reader)?; // varnames
            MPKFileReader::read_object(reader)?; // freevars
            MPKFileReader::read_object(reader)?; // cellvars
            let file_name = MPKFileReader::read_object(reader)?; // filename
            let name = MPKFileReader::read_object(reader)?; // name

            // eprintln!("{}", std::str::from_utf8(&file_name)?);

            reader.read_u32::<LittleEndian>()?;
            MPKFileReader::read_object(reader)?; // lnotab

            Ok(file_name)
        } else {
            eprintln!("{} {}", t, reader.position());
            Ok(vec![])
        }
    }

    pub fn extract_files<P: AsRef<std::path::Path>>(&self, out_dir: P) -> anyhow::Result<()> {
        use indicatif::ProgressBar;

        let bar = ProgressBar::new(self.files.len() as u64);

        std::fs::create_dir_all(&out_dir)?;

        let basename = self.path.file_stem().unwrap().to_str().unwrap();
        let parent_path = self.path.parent().unwrap();

        let mut mpk_map = HashMap::new();
        for file in &self.files {
            if !file.is_folder {
                let mpk_file = mpk_map.entry(&file.file_number).or_try_insert_with(|| {
                    let mpk_file = if file.file_number == 0 {
                        format!("{}.mpk", basename)
                    } else {
                        format!("{}{}.mpk", basename, file.file_number)
                    };
                    std::fs::File::open(parent_path.join(mpk_file))
                })?;

                let offset = mpk_file.seek(SeekFrom::Start(file.offset.into()))?;
                assert!(offset == file.offset.into());
                let mut file_buffer = vec![0; file.size as usize];
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
                        let mut buffer = vec![0; 0 as usize];
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
                    } else {
                        (file_buffer, None)
                    }
                } else if &magic[..2] == b"\xE2\x06" {
                    // This is a mangled zlib compressed file
                    // TODO(alexander): Move handling of these to a new crate
                    // TODO(alexander): Reduce number of vec allocations
                    reader.seek(SeekFrom::Start(0))?;
                    let mut buffer = vec![0; 0 as usize];
                    reader.read_to_end(&mut buffer)?;

                    let offset = (buffer.len() - 8) % 37;
                    let end = 128 - offset;
                    let end = end.min(buffer.len());
                    // eprintln!("{} {} {}", buffer.len(), offset, end);
                    let head = &mut buffer[..end];
                    for x in head.iter_mut() {
                        *x = *x ^ 154;
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
                    let is_pyc = &result_buffer[..4] == b"\x03\xf3\r\n";

                    let result_buffer = if is_pyc {
                        // Extract filename
                        let mut reader = std::io::Cursor::new(&result_buffer);
                        reader.read_u32::<LittleEndian>()?;
                        reader.read_u32::<LittleEndian>()?;
                        let file_name = MPKFileReader::read_object(&mut reader)?; // filename
                        let file_name = std::str::from_utf8(&file_name)?;
                        (result_buffer, Some(format!("Script/Python/{}c", file_name)))
                    } else {
                        (result_buffer, None)
                    };
                    result_buffer
                } else {
                    (file_buffer, None)
                };

                let file_name = if let Some(file_name) = alt_file_name {
                    file_name
                } else {
                    file.name.clone()
                };
                let out_file_path = out_dir.as_ref().join(file_name);
                std::fs::create_dir_all(out_file_path.parent().unwrap())?;

                let mut out_file = std::fs::File::create(&out_file_path)
                    .context(out_file_path.display().to_string())?;
                out_file.write_all(&file_buffer)?;
            } else {
                let out_file_path = out_dir.as_ref().join(file.name.clone());
                std::fs::create_dir_all(out_file_path)?;
            }

            bar.inc(1);
        }

        bar.finish();

        Ok(())
    }
}
