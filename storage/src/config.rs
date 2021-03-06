use std::path::{PathBuf};
use std::{fs};
use cardano::block::EpochId;

use cardano::util::hex;

use types::*;

#[derive(Clone)]
pub struct StorageConfig {
    pub root_path: PathBuf
}

impl StorageConfig {
    pub fn new(path_buf: &PathBuf) -> Self {
        StorageConfig { root_path: path_buf.clone() }
    }
    pub fn get_path(&self) -> PathBuf {
        self.root_path.clone()
    }
    pub fn get_filetype_dir(&self, ft: StorageFileType) -> PathBuf {
        let mut p = self.get_path();
        match ft {
            StorageFileType::RefPack => p.push("refpack/"),
            StorageFileType::Pack => p.push("pack/"),
            StorageFileType::Index => p.push("index/"),
            StorageFileType::Blob => p.push("blob/"),
            StorageFileType::Tag => p.push("tag/"),
            StorageFileType::Epoch => p.push("epoch/"),
        }
        p
    }
    pub fn get_config_file(&self) -> PathBuf {
        let mut p = self.get_path();
        p.push("config.yml");
        p
    }
    pub fn get_pack_filepath(&self, packhash: &PackHash) -> PathBuf {
        let mut p = self.get_filetype_dir(StorageFileType::Pack);
        p.push(hex::encode(packhash));
        p
    }
    pub fn get_index_filepath(&self, packhash: &PackHash) -> PathBuf {
        let mut p = self.get_filetype_dir(StorageFileType::Index);
        p.push(hex::encode(packhash));
        p
    }
    pub fn get_blob_filepath(&self, blockhash: &BlockHash) -> PathBuf {
        let mut p = self.get_filetype_dir(StorageFileType::Blob);
        p.push(hex::encode(blockhash));
        p
    }
    pub fn get_tag_filepath<P: AsRef<str>>(&self, s: P) -> PathBuf {
        let mut p = self.get_filetype_dir(StorageFileType::Tag);
        p.push(s.as_ref());
        p
    }
    pub fn get_refpack_filepath<S: AsRef<str>>(&self, name: S) -> PathBuf {
        let mut p = self.get_filetype_dir(StorageFileType::RefPack);
        p.push(name.as_ref());
        p
    }
    pub fn get_epoch_dir(&self, epoch: EpochId) -> PathBuf {
        let mut p = self.get_filetype_dir(StorageFileType::Epoch);
        p.push(epoch.to_string());
        p
    }

    pub fn get_epoch_pack_filepath(&self, epoch: EpochId) -> PathBuf {
        let mut p = self.get_epoch_dir(epoch);
        p.push("pack");
        p
    }
    pub fn get_epoch_refpack_filepath(&self, epoch: EpochId) -> PathBuf {
        let mut p = self.get_epoch_dir(epoch);
        p.push("refpack");
        p
    }

    pub fn list_indexes(&self) -> Vec<PackHash> {
        let mut packs = Vec::new();
        let p = self.get_filetype_dir(StorageFileType::Index);
        for entry in fs::read_dir(p).unwrap() {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_file() {
                if let Ok(s) = entry.file_name().into_string() {
                    if s.len() == 64 {
                        let v = hex::decode(s.as_ref()).unwrap();
                        let mut packref = [0;HASH_SIZE];
                        packref.clone_from_slice(&v[..]);
                        packs.push(packref);
                    }
                }
            }
        }
        packs
    }

    pub fn list_blob(&self, limits: Option<u32>) -> Vec<BlockHash> {
        let mut blobs = Vec::new();
        let p = self.get_filetype_dir(StorageFileType::Blob);
        for entry in fs::read_dir(p).unwrap() {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_file() {
                if let Ok(s) = entry.file_name().into_string() {
                    if s.len() == 64 {
                        let v = hex::decode(s.as_ref()).unwrap();
                        let mut blobref = [0;HASH_SIZE];
                        blobref.clone_from_slice(&v[..]);
                        blobs.push(blobref);
                        if blobs.len() == 0xffffffff { break };
                        match limits {
                            None => {},
                            Some(l) => if blobs.len() as u32 >= l { break }
                        }
                    }
                }
            }
        }
        blobs
    }
}
