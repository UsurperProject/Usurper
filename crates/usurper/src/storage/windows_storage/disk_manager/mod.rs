use crate::storage::DiskManager;

impl DiskManager {
    fn get_disks() -> Vec<(usize, String)>;
    fn get_partitions(disk: Option<usize>) -> Vec<(usize, PartitionInfo)>;
    fn select_partition(disk: usize, partition: usize) -> PartitionManager;
}