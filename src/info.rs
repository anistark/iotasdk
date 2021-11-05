// All Info
use sysinfo::{System, SystemExt};

pub fn sys() {
    // println!("sys info");
    let mut sys = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();

    // // We display all disks' information:
    // println!("=> disks:");
    // for disk in sys.disks() {
    //     println!("{:?}", disk);
    // }

    // // Network interfaces name, data received and data transmitted:
    // println!("=> networks:");
    // for (interface_name, data) in sys.networks() {
    //     println!("{}: {}/{} B", interface_name, data.received(), data.transmitted());
    // }

    // // Components temperature:
    // println!("=> components:");
    // for component in sys.components() {
    //     println!("{:?}", component);
    // }

    // println!("=> system:");
    // RAM and swap information:
    println!("Avaialble memory: {} GB", (sys.total_memory()-sys.used_memory())/1024);

    // Display system information:
    println!("System name:             {:?}", sys.name());
    println!("System kernel version:   {:?}", sys.kernel_version());
    println!("System OS version:       {:?}", sys.os_version());
    println!("System host name:        {:?}", sys.host_name());

    // Number of processors:
    println!("NB processors: {}", sys.processors().len());

    // Display processes ID, name na disk usage:
    // for (pid, process) in sys.processes() {
    //     println!("[{}] {} {:?}", pid, process.name(), process.disk_usage());
    // }
}
