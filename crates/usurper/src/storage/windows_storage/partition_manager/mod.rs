use crate::storage::PartitionManagerTrait;
pub struct PartitionManager;

impl PartitionManager {
    fn new(disk: usize, partition: usize) -> Self;
    fn format(&self ,format: PartitionFormats) -> std::io::Result<()>;
    fn shrink(&self ,format: PartitionFormats) -> std::io::Result<PartitionManager>;
    fn get_info(&self) -> PartitionInfo;
}