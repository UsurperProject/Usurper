use input_macro::input;
use usurper::device::{Device, PartitionTypes, PartitionFormats, Distro};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let device = Device::new();
    println!("Platform: {} | Hostname: {}", device.platform, device.hostname);

    let installer = device.mount_installer("debian.iso"); // Installer 
    let distro_base = installer.detect_distro().base();

    if ![Distro::Debian /* , Distro::Fedora */].contains(&distro_base) { // This will be fixed
        panic!("Unsupported distribution detected".into());
    }

    let disk_man = device.create_disk_manager(); // DiskManager    
    let partitions = disk_man.get_partitions();
    for p in &partitions {
        println!("{}: {}    {}GB", p.serial_id, p.label, p.size as f32 /1024.0);
    }

    let partition_serial: u32 = input!("Select a partition serial_id: ").parse()?;
    let mut root_partition = disk_man.select_partition(partition_serial)?;

    // actual partitioning part
    root_partition.format(PartitionTypes::Ext4)?;
    let mut boot_partition = root_partition.shrink(2048, PartitionFormats::Fat32)?;
    let mut swap_partition = root_partition.shrink(4096, PartitionFormats::Swap)?;

    // Installation is async so this is fine (i think)
    installer.install_to(root_partition, boot_partition, swap_partition);
    
    println!("Installation started...");
    sleep(std::time::Duration::from_secs(5));

    while let Some(status) = installer.get_status() {
        if status.complete || status.crash {
            break;
        }

        print!("\x1B[2J\x1B[1;1H");
        println!("Progress: {}% | Speed: {}Mbps | ETA: {}", 
            status.progress, status.speed, status.eta);
        
        sleep(std::time::Duration::from_secs(1));
    }

    println!("Installation completo");
    // user config and boot config will come next, but... yea
    Ok(())
}