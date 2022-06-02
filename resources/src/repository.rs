use std::io::{BufRead, BufReader, Read};

use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Default)]
pub struct File {
    unk1: u16,
    unk2: u16,
    flag: u8,
    hash: [u8; 16],
    name: String,
    folder_index: u16,
    type_index: u16,
    related_hashes: Vec<[u8; 16]>,

    folder_path: String,
    type_name: String,
}

impl File {
    pub fn hash_file_name(&self) -> String {
        format!(
            "{:02x}/{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            self.hash[0],
            self.hash[0],
            self.hash[1],
            self.hash[2],
            self.hash[3],
            self.hash[4],
            self.hash[5],
            self.hash[6],
            self.hash[7],
            self.hash[8],
            self.hash[9],
            self.hash[10],
            self.hash[11],
            self.hash[12],
            self.hash[13],
            self.hash[14],
            self.hash[15]
        )
    }

    pub fn file_path(&self) -> String {
        std::path::Path::new(&self.folder_path)
            .join(format!(
                "{}.{}",
                self.name.replace("\\", "_").replace(":", "_"),
                self.type_name
            ))
            .as_os_str()
            .to_string_lossy()
            .to_string()
    }
}

pub struct Repository {
    version: u32,
    files: Vec<File>,
    resource_types: Vec<String>,
    folder_paths: Vec<String>,
}

impl Repository {
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<Self> {
        let file = std::fs::File::open(path)?;
        let mut reader = BufReader::new(&file);

        let version = reader.read_u32::<LittleEndian>()?;
        let _flag1 = reader.read_u16::<LittleEndian>()?;
        let _flag2 = reader.read_u32::<LittleEndian>()?;

        let size_of_types = reader.read_u16::<LittleEndian>()? as usize;
        let mut resource_types = vec![0; size_of_types];
        reader.read_exact(&mut resource_types)?;
        let resource_types: Vec<&str> = std::str::from_utf8(&resource_types)?.split(";").collect();

        let size_of_paths = reader.read_u16::<LittleEndian>()? as usize;
        let mut folder_paths = vec![0; size_of_paths];
        reader.read_exact(&mut folder_paths)?;
        let folder_paths: Vec<&str> = std::str::from_utf8(&folder_paths)?.split(";").collect();

        let mut files: Vec<File> = Vec::new();
        while reader.fill_buf().map(|b| !b.is_empty())? {
            //
            let mut file: File = Default::default();
            file.unk1 = reader.read_u16::<LittleEndian>()?;
            file.unk2 = reader.read_u16::<LittleEndian>()?;
            file.flag = reader.read_u8()?;
            reader.read_exact(&mut file.hash)?;

            let file_name_size = reader.read_u16::<LittleEndian>()?;
            let mut file_name = vec![0; file_name_size as usize];
            reader.read_exact(&mut file_name)?;
            file.name = std::str::from_utf8(&file_name)?.to_string();

            file.folder_index = reader.read_u16::<LittleEndian>()?;
            file.type_index = reader.read_u16::<LittleEndian>()?;

            let related_hash_count = reader.read_u16::<LittleEndian>()?;
            for _ in 0..related_hash_count {
                let mut hash: [u8; 16] = Default::default();
                reader.read_exact(&mut hash)?;
                file.related_hashes.push(hash);
            }

            file.folder_path = folder_paths[file.folder_index as usize].to_string();
            file.type_name = resource_types[file.type_index as usize].to_string();

            files.push(file);
        }

        Ok(Self {
            version,
            files,
            resource_types: resource_types.into_iter().map(|a| a.to_string()).collect(),
            folder_paths: folder_paths.into_iter().map(|a| a.to_string()).collect(),
        })
    }

    pub fn files(&self) -> &Vec<File> {
        &self.files
    }
}
