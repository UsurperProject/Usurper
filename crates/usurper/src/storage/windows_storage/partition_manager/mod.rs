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

pub struct PartitionManager {
    pub serial_id: usize,
    pub uuid: Uuid,
    pub size_mb: usize,
    pub format: PartitionFormats,
    pub name: String, 
};

impl PartitionManager {
    fn new(disk: usize, partition: usize) -> Self;
    fn format(&self ,format: PartitionFormats) -> std::io::Result<()>;
    fn shrink(&self ,format: PartitionFormats) -> std::io::Result<PartitionManager>;
}