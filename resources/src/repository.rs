use std::io::{BufRead, BufReader, Read};

use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Default)]
pub struct File {
    unk1: u16,
    unk2: u16,
    flag: u8,
    uuid: [u8; 16],
    name: String,
    folder_index: u16,
    type_index: u16,
    dependent_resources: Vec<[u8; 16]>,

    folder_path: String,
    type_name: String,
}

impl File {
    pub fn uuid_file_name(&self) -> String {
        format!(
            "{:02x}/{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            self.uuid[0],
            self.uuid[0],
            self.uuid[1],
            self.uuid[2],
            self.uuid[3],
            self.uuid[4],
            self.uuid[5],
            self.uuid[6],
            self.uuid[7],
            self.uuid[8],
            self.uuid[9],
            self.uuid[10],
            self.uuid[11],
            self.uuid[12],
            self.uuid[13],
            self.uuid[14],
            self.uuid[15]
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
        let size_of_paths = if size_of_paths == 0xFFFF {
            reader.read_u32::<LittleEndian>()? as usize
        } else {
            size_of_paths
        };
        let mut folder_paths = vec![0; size_of_paths];
        reader.read_exact(&mut folder_paths)?;
        let folder_paths: Vec<&str> = std::str::from_utf8(&folder_paths)?.split(";").collect();

        let mut files: Vec<File> = Vec::new();
        while reader.fill_buf().map(|b| !b.is_empty())? {
            //
            // TODO(alexander): Move this to file and have a from_reader thing or something
            let mut file: File = Default::default();
            file.unk1 = reader.read_u16::<LittleEndian>()?;
            file.unk2 = reader.read_u16::<LittleEndian>()?;
            file.flag = reader.read_u8()?;
            reader.read_exact(&mut file.uuid)?;

            let file_name_size = reader.read_u16::<LittleEndian>()?;
            let mut file_name = vec![0; file_name_size as usize];
            reader.read_exact(&mut file_name)?;
            file.name = std::str::from_utf8(&file_name)?.to_string();

            file.folder_index = reader.read_u16::<LittleEndian>()?;
            file.type_index = reader.read_u16::<LittleEndian>()?;

            let dependent_uuids_count = reader.read_u16::<LittleEndian>()?;
            for _ in 0..dependent_uuids_count {
                let mut uuid: [u8; 16] = Default::default();
                reader.read_exact(&mut uuid)?;
                file.dependent_resources.push(uuid);
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
