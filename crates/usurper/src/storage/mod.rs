#[cfg(target_os = "windows")]
mod windows_storage;

#[cfg(target_os = "windows")]
pub use windows_storage::*;

#[cfg(target_os = "windows")]
mod linux_storage;

#[cfg(target_os = "windows")]
pub use linux_storage::*;

use uuid::Uuid;

pub enum PartitionFormats {
    Ntfs,
    Ext4,
    Btrfs,
    Zfs,
    Fat,
    Fat32,
    Fat64,
    Raw,
    Swao
}
pub struct PartitionInfo {
    pub uuid: Uuid,
    pub size_mb: usize,
    pub format: PartitionFormats,
    pub name: String, 
}

