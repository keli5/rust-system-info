use sysinfo::*;
use text_io::read;

fn main() {
    let mut system = sysinfo::System::new_all();
    system.refresh_all();
    println!("Info to show [CPU/disk]:");
    let mut line: String = read!("{}\n");

    line = line.to_lowercase();

    if line == "" {
        line = String::from("cpu");
    }

    if line == "cpu" {
        cpuinfo(system)
    } else if line == "disk" {
        diskinfo(system)
    } else {
        println!("Invalid choice recieved.")
    }
}

fn cpuinfo(system: sysinfo::System) {
    println!("{} cores", system.get_processors().len());

    for cpu in system.get_processors() {
        let cpunum = cpu.get_name().replace("cpu", "");
        let mut usage = cpu.get_cpu_usage() * 100.;
        usage = usage.round() / 100.;
        println!("Core #{}: {}% usage", cpunum, usage)
    }
}

fn diskinfo(system: sysinfo::System) {
    for disk in system.get_disks() {
        let totalsize = disk.get_total_space() as i64;
        let freespace = disk.get_available_space() as i64;
        // TODO: disk.get_name() returns "" for physical disks on windows

        println!("Disk: {:?}", disk.get_name());
        println!("--- Type: {:?}", disk.get_type());
        println!("--- Mountpoint: {:?}", disk.get_mount_point());
        println!("--- Total size: {} GB", totalsize / 1e+9 as i64 );
        println!("--- Free space: {} GB", freespace / 1e+9 as i64 );
        print!("\n")
    }
}