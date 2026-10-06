use usurper::device;

fn main() {
    let device = device::Device::new();
    print!("{} : {}", device.platform, device.hostname);
}

