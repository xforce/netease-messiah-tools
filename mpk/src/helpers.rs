use std::io::{BufReader, Read, Seek};
use byteorder::{LittleEndian, ReadBytesExt};
use crate::{helpers, MPKFileReader};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub (crate) enum PythonVersion {
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

pub(crate) fn read_py_object(
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
                let n = read_py_object(reader, version)?;
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
                let n = read_py_object(reader, version)?;
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
                let n = read_py_object(reader, version)?;
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

            read_py_object(reader, version)?; // code
            let _file_name_consts = read_py_object(reader, version)?; // consts
            read_py_object(reader, version)?; // names
            // 3.11
            let file_name = if version == PythonVersion::Version3_11 {
                read_py_object(reader, version)?; // localsplusnames
                read_py_object(reader, version)?; // localspluskinds
                let file_name = read_py_object(reader, version)?; // filename
                read_py_object(reader, version)?; // name
                read_py_object(reader, version)?; // qualname
                reader.read_u32::<LittleEndian>()?; // firstlineno
                read_py_object(reader, version)?; // linetable
                read_py_object(reader, version)?; // exceptiontable
                file_name
            } else if version == PythonVersion::Version2_7 {
                read_py_object(reader, version)?; // varnames
                read_py_object(reader, version)?; // freevars
                read_py_object(reader, version)?; // cellvars
                let file_name = read_py_object(reader, version)?; // filename
                let _name = read_py_object(reader, version)?; // name

                reader.read_u32::<LittleEndian>()?;
                read_py_object(reader, version)?; // lnotab
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

pub(crate) fn detect_python_version_from_py_header(
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

pub(crate) fn file_name_from_py_buffer(buffer: &Vec<u8>) -> anyhow::Result<String> {
    let python_version =
        helpers::detect_python_version_from_py_header(&buffer)?;
    if let Some(python_version) = python_version {
        // Extract filename
        let new_file_name = match python_version {
            PythonVersion::Version2_7 => {
                let mut reader = std::io::Cursor::new(buffer);
                reader.read_u32::<LittleEndian>()?;
                reader.read_u32::<LittleEndian>()?;
                let file_name =
                    helpers::read_py_object(&mut reader, python_version)?; // filename
                file_name
            }
            PythonVersion::Version3_11 => {
                let mut reader = std::io::Cursor::new(buffer);
                reader.read_u32::<LittleEndian>()?;
                reader.read_u32::<LittleEndian>()?;
                let file_name =
                    helpers::read_py_object(&mut reader, python_version)?; // filename
                file_name
            }
            _ => anyhow::bail!("Invalid python version"),
        };
        Ok(std::str::from_utf8(&new_file_name)?.to_string())
    } else {
        anyhow::bail!("Invalid python version");
    }
}