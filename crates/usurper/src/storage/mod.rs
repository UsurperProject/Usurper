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

pub trait DiskManagerTrait {
    fn get_disks() -> Vec<(usize, String)>;
    fn get_partitions(disk: usize) -> Vec<(usize, PartitionInfo)>;
    fn select_partition(disk: usize, partition: usize) -> PartitionManager;
}

pub trait PartitionManagerTrait {
    fn new(disk: usize, partition: usize) -> Self;
    fn format(&self ,format: PartitionFormats) -> std::io::Result<()>;
    fn shrink(&self ,format: PartitionFormats) -> std::io::Result<PartitionManager>;
    fn get_info(&self) -> PartitionInfo;
}