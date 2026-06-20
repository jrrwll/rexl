use crate::convert::{EXE_HEAD, JPG_HEAD, MOV_HEAD, MP4_HEAD};
use crate::convert::{disguise, disguise_with_mask_file, reveal};

#[derive(Clone, Debug)]
pub enum DisguiseType {
    Jpg,
    Mov,
    Mp4,
    Exe,
    MaskFile(String),
}

pub struct Apate {
    pub disguise_type: DisguiseType,
    pub dry_run: bool,
}

impl Apate {
    pub fn disguise(&self, file_path: &str) -> Result<usize, String> {
        let head = match self.disguise_type {
            DisguiseType::Jpg => JPG_HEAD,
            DisguiseType::Mov => MOV_HEAD,
            DisguiseType::Mp4 => MP4_HEAD,
            DisguiseType::Exe => EXE_HEAD,
            DisguiseType::MaskFile(ref mask_path) => {
                return disguise_with_mask_file(file_path, mask_path, self.dry_run)
                    .map_err(|e| e.to_string());
            }
        };
        disguise(file_path, head, self.dry_run).map_err(|e| e.to_string())
    }

    pub fn reveal(&self, file_path: &str) -> Result<usize, String> {
        reveal(file_path, self.dry_run).map_err(|e| e.to_string())
    }
}
