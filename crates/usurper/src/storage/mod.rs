#[cfg(target_os = "windows")]
mod windows_storage;

#[cfg(target_os = "windows")]
pub use windows_storage::*;

#[cfg(target_os = "linux")]
mod linux_storage;

#[cfg(target_os = "linux")]
pub use linux_storage::*;

use uuid::Uuid;

pub struct DiskManager {
    disks_count: usize
}

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

pub struct PartitionManager {
    pub serial_id: usize,
    pub uuid: Uuid,
    pub size_mb: usize,
    pub format: PartitionFormats,
    pub label: String, 
}