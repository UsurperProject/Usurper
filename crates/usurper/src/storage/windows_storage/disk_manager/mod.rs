use crate::storage::DiskManagerTrait;

pub struct DiskManager {

}

impl DiskManager {
    fn get_disks() -> Vec<(usize, String)>;
    fn get_partitions(disk: usize) -> Vec<(usize, PartitionInfo)>;
    fn select_partition(disk: usize, partition: usize) -> PartitionManager;
}