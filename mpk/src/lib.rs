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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum PythonVersion {
    Version2_0,
    Version2_1,
    Version2_2,
    Version2_3,
    Version2_4,
    Version2_5,
    Version2_6,
    Version2_7,
    Version3_0,
    Version3_1,
    Version3_2,
    Version3_3,
    Version3_4,
    Version3_5,
    Version3_6,
    Version3_7,
    Version3_8,
    Version3_9,
    Version3_10,
    Version3_11,
    Version3_12,
    Version3_13,
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

    fn read_object(
        reader: &mut std::io::Cursor<&Vec<u8>>,
        version: PythonVersion,
    ) -> anyhow::Result<Vec<u8>> {
        const TYPE_NULL: u8 = b'0';
        const TYPE_NONE: u8 = b'N';
        const TYPE_FALSE: u8 = b'F';
        const TYPE_TRUE: u8 = b'T';
        const TYPE_STOPITER: u8 = b'S';
        const TYPE_ELLIPSIS: u8 = b'.';
        const TYPE_INT: u8 = b'i';
        const TYPE_FLOAT: u8 = b'f';
        const TYPE_BINARY_FLOAT: u8 = b'g';
        const TYPE_COMPLEX: u8 = b'x';
        const TYPE_BINARY_COMPLEX: u8 = b'y';
        const TYPE_LONG: u8 = b'l';
        const TYPE_STRING: u8 = b's';
        const TYPE_INTERNED: u8 = b't';
        const TYPE_REF: u8 = b'r';
        const TYPE_TUPLE: u8 = b'(';
        const TYPE_LIST: u8 = b'[';
        const _TYPE_DICT: u8 = b'{';
        const TYPE_CODE: u8 = b'c';
        const TYPE_UNICODE: u8 = b'u';
        const _TYPE_UNKNOWN: u8 = b'?';
        const TYPE_SET: u8 = b'<';
        const TYPE_FROZENSET: u8 = b'>';
        const TYPE_ASCII: u8 = b'a';
        const TYPE_ASCII_INTERNED: u8 = b'A';
        const TYPE_SMALL_TUPLE: u8 = b')';
        const TYPE_SHORT_ASCII: u8 = b'z';
        const TYPE_SHORT_ASCII_INTERNED: u8 = b'Z';

        let t = reader.read_u8()?;
        let t = t & 0x7F;
        match t {
            TYPE_BINARY_COMPLEX => {
                let _ = reader.read_u64::<LittleEndian>()?;
                let _ = reader.read_u64::<LittleEndian>()?;
                Ok(vec![])
            }
            TYPE_LONG => {
                let m_size = reader.read_i32::<LittleEndian>()?;
                let actual_size = if m_size >= 0 { m_size } else { -m_size };
                for _ in 0..actual_size {
                    reader.read_u16::<LittleEndian>()?;
                }
                Ok(vec![])
            }
            TYPE_STRING => {
                let l = reader.read_u32::<LittleEndian>()?;
                let mut buf = vec![0; l as usize];
                reader.read_exact(&mut buf)?;
                Ok(buf)
            }
            TYPE_UNICODE => {
                let l = reader.read_u32::<LittleEndian>()?;
                let mut buf = vec![0; l as usize];
                reader.read_exact(&mut buf)?;
                Ok(buf)
            }
            TYPE_INTERNED => {
                let l = reader.read_u32::<LittleEndian>()?;
                let mut buf = vec![0; l as usize];
                reader.read_exact(&mut buf)?;
                Ok(buf)
            }
            TYPE_TUPLE | TYPE_LIST => {
                let l = reader.read_u32::<LittleEndian>()?;
                let mut name = vec![];
                for _ in 0..l {
                    let n = MPKFileReader::read_object(reader, version)?;
                    if !n.is_empty() {
                        name = n;
                    }
                }
                Ok(name)
            }
            TYPE_SMALL_TUPLE => {
                let l = reader.read_u8()?;
                let mut name = vec![];
                for _ in 0..l {
                    let n = MPKFileReader::read_object(reader, version)?;
                    if !n.is_empty() {
                        name = n;
                    }
                }
                Ok(name)
            }
            TYPE_BINARY_FLOAT => {
                let _ = reader.read_u64::<LittleEndian>()?;
                Ok(vec![])
            }
            TYPE_FLOAT => {
                let l = reader.read_u8()?;
                reader.seek_relative(l as i64)?;
                Ok(vec![])
            }
            TYPE_COMPLEX => {
                let l = reader.read_u8()?;
                reader.seek_relative(l as i64)?;
                let l = reader.read_u8()?;
                reader.seek_relative(l as i64)?;
                Ok(vec![])
            }
            TYPE_INT | b'R' => {
                let _ = reader.read_u32::<LittleEndian>()?;
                Ok(vec![])
            }
            TYPE_REF => {
                let _ = reader.read_u32::<LittleEndian>()?;
                Ok(vec![])
            }
            TYPE_SHORT_ASCII | TYPE_SHORT_ASCII_INTERNED => {
                let l = reader.read_u8()?;
                let mut buf = vec![0; l as usize];
                reader.read_exact(&mut buf)?;
                Ok(buf)
            }
            TYPE_ASCII | TYPE_ASCII_INTERNED => {
                let l = reader.read_u32::<LittleEndian>()?;
                let mut buf = vec![0; l as usize];
                reader.read_exact(&mut buf)?;
                Ok(buf)
            }
            TYPE_FROZENSET | TYPE_SET => {
                let n = reader.read_u32::<LittleEndian>()?;
                let mut name = vec![];
                for _ in 0..n {
                    let n = MPKFileReader::read_object(reader, version)?;
                    if !n.is_empty() {
                        name = n;
                    }
                }
                Ok(name)
            }
            TYPE_NONE | TYPE_TRUE | TYPE_FALSE | TYPE_ELLIPSIS | TYPE_NULL | TYPE_STOPITER => {
                Ok(vec![])
            }
            TYPE_CODE => {
                let _argcount = reader.read_u32::<LittleEndian>()?; // argcount
                let _posonlyargcount = reader.read_u32::<LittleEndian>()?;
                let _kwonlyargcount = reader.read_u32::<LittleEndian>()?;
                let _stacksize = reader.read_u32::<LittleEndian>()?;

                // 3.11
                if version == PythonVersion::Version3_11 {
                    let _flags = reader.read_u32::<LittleEndian>()?;
                }

                MPKFileReader::read_object(reader, version)?; // code
                let _file_name_consts = MPKFileReader::read_object(reader, version)?; // consts
                MPKFileReader::read_object(reader, version)?; // names
                                                              // 3.11
                let file_name = if version == PythonVersion::Version3_11 {
                    MPKFileReader::read_object(reader, version)?; // localsplusnames
                    MPKFileReader::read_object(reader, version)?; // localspluskinds
                    let file_name = MPKFileReader::read_object(reader, version)?; // filename
                    MPKFileReader::read_object(reader, version)?; // name
                    MPKFileReader::read_object(reader, version)?; // qualname
                    reader.read_u32::<LittleEndian>()?; // firstlineno
                    MPKFileReader::read_object(reader, version)?; // linetable
                    MPKFileReader::read_object(reader, version)?; // exceptiontable
                    file_name
                } else if version == PythonVersion::Version2_7 {
                    MPKFileReader::read_object(reader, version)?; // varnames
                    MPKFileReader::read_object(reader, version)?; // freevars
                    MPKFileReader::read_object(reader, version)?; // cellvars
                    let file_name = MPKFileReader::read_object(reader, version)?; // filename
                    let _name = MPKFileReader::read_object(reader, version)?; // name

                    reader.read_u32::<LittleEndian>()?;
                    MPKFileReader::read_object(reader, version)?; // lnotab
                    file_name
                } else {
                    vec![]
                };

                let file_name = if file_name.is_empty() {
                    file_name
                } else {
                    file_name
                };
                Ok(file_name)
            }
            _ => {
                panic!("Error {}", t);
            }
        }
    }

    fn detect_python_version_from_py_header(
        buffer: &[u8],
    ) -> anyhow::Result<Option<PythonVersion>> {
        let version_ranges = [
            (50823, 50823, PythonVersion::Version2_0),
            (60202, 60202, PythonVersion::Version2_1),
            (60717, 60717, PythonVersion::Version2_2),
            (62011, 62021, PythonVersion::Version2_3),
            (62041, 62061, PythonVersion::Version2_4),
            (62071, 62131, PythonVersion::Version2_5),
            (62151, 62161, PythonVersion::Version2_6),
            (62171, 62211, PythonVersion::Version2_7),
            (3000, 3131, PythonVersion::Version3_0),
            (3141, 3151, PythonVersion::Version3_1),
            (3160, 3180, PythonVersion::Version3_2),
            (3190, 3230, PythonVersion::Version3_3),
            (3250, 3310, PythonVersion::Version3_4),
            (3320, 3351, PythonVersion::Version3_5),
            (3360, 3379, PythonVersion::Version3_6),
            (3390, 3399, PythonVersion::Version3_7),
            (3400, 3419, PythonVersion::Version3_8),
            (3420, 3429, PythonVersion::Version3_9),
            (3430, 3449, PythonVersion::Version3_10),
            (3450, 3499, PythonVersion::Version3_11),
            (3500, 3549, PythonVersion::Version3_12),
            (3550, 3599, PythonVersion::Version3_13),
        ];

        let mut reader = BufReader::new(std::io::Cursor::new(&buffer));
        let value = reader.read_u16::<LittleEndian>()?;
        if &buffer[2..4] != b"\r\n" {
            return Ok(None);
        }
        for (version_min, version_max, version) in version_ranges {
            if value >= version_min && value <= version_max {
                return Ok(Some(version));
            }
        }
        return Ok(None);
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
                        (decompressed, None)
                    } else {
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

                    // Check the decoded magic for pyc, if it's pyc we would like to move this file elsewhere
                    let python_version =
                        Self::detect_python_version_from_py_header(&result_buffer)?;
                    if let Some(python_version) = python_version {
                        // Extract filename
                        let new_file_name = match python_version {
                            PythonVersion::Version2_7 => {
                                let mut reader = std::io::Cursor::new(&result_buffer);
                                reader.read_u32::<LittleEndian>()?;
                                reader.read_u32::<LittleEndian>()?;
                                let file_name =
                                    MPKFileReader::read_object(&mut reader, python_version)?; // filename
                                file_name
                            }
                            PythonVersion::Version3_11 => {
                                let mut reader = std::io::Cursor::new(&result_buffer);
                                reader.read_u32::<LittleEndian>()?;
                                reader.read_u32::<LittleEndian>()?;
                                let file_name =
                                    MPKFileReader::read_object(&mut reader, python_version)?; // filename
                                file_name
                            }
                            _ => vec![],
                        };

                        if new_file_name.is_empty() {
                            (result_buffer, None)
                        } else {
                            let new_file_name = std::str::from_utf8(&new_file_name)?;
                            (
                                result_buffer,
                                Some(format!("Script/Python/{}c", new_file_name)),
                            )
                        }
                    } else {
                        (result_buffer, None)
                    }
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
