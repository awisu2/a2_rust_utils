use std::{
    fs::File,
    io::{BufReader, Read},
};

use anyhow::Result;
use zip::ZipArchive;

use crate::file::{domain::zip_infos::ZipInfo, FileInfo};

pub struct ZipEx {}

impl ZipEx {
    pub fn open(path: &str) -> Result<ZipArchive<BufReader<File>>> {
        let file = File::open(path)?;
        let read = BufReader::new(file);
        let archive = ZipArchive::new(read)?;
        Ok(archive)
    }

    pub fn read(path: &str) -> Result<(ZipArchive<BufReader<File>>, Vec<ZipInfo>)> {
        let mut archive = Self::open(path)?;
        let mut infos: Vec<ZipInfo> = Vec::new();
        for i in 0..archive.len() {
            let entry = archive.by_index(i)?;
            let info = ZipInfo::new(i, path, entry.name()).set_metas(entry);
            infos.push(info);
        }

        Ok((archive, infos))
    }

    pub fn read_file_infos(path: &str) -> Result<(ZipArchive<BufReader<File>>, Vec<FileInfo>)> {
        let (archive, zip_infos) = Self::read(path)?;
        let infos = zip_infos
            .iter()
            .map(|zip_info| FileInfo::from(zip_info))
            .collect::<Vec<FileInfo>>();
        Ok((archive, infos))
    }

    pub fn read_bytes(
        buffer: &mut ZipArchive<BufReader<File>>,
        file_path: &str,
    ) -> Result<Vec<u8>> {
        let mut file = buffer.by_name(file_path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        Ok(buf)
    }
}
