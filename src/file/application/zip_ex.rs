use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
};

use anyhow::Result;
use zip::ZipArchive;

use crate::file::{domain::zip_infos::ZipInfo, FileInfo, PathEx};

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

        // zip 形式では、dir はファイルとして存在しないことがあるため、infos から dir を補完する
        {
            let mut real_dirs: HashMap<String, ZipInfo> = HashMap::new();
            let mut missing_dirs: HashMap<String, ZipInfo> = HashMap::new();
            for info in &mut infos {
                if info.is_dir {
                    real_dirs.insert(info.name.clone(), info.clone());
                    if missing_dirs.contains_key(&info.name) {
                        missing_dirs.remove(&info.name);
                    }
                } else if let Some((dir_path, _)) = info.name.rsplit_once('/') {
                    if !real_dirs.contains_key(dir_path) {
                        missing_dirs
                            .insert(dir_path.to_string_ex(), ZipInfo::new_dir(path, dir_path));
                    }
                }
            }

            infos.extend(missing_dirs.values().cloned());
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

// test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_zip() {
        let path = "tests/data/sample.zip";

        let (_, infos) = ZipEx::read(path).unwrap();
        assert!(infos.len() > 0);

        let (_, infos) = ZipEx::read_file_infos(path).unwrap();
        assert!(infos.len() > 0);
    }
}
