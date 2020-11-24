use sysinfo::*;
use text_io::read;
mod utility;
use utility::roundplaces;

fn main() {
    let mut system = sysinfo::System::new_all();
    system.refresh_all();
    println!("Info to show [cpu/disk/mem]:");
    let mut line: String = read!("{}\n");

    line = line.to_lowercase();

    if line == "" {
        line = String::from("cpu");
    }

    if line == "cpu" {
        cpuinfo(system)
    } else if line == "disk" {
        diskinfo(system)
    } else if line == "mem" {
        meminfo(system)
    } else {
        println!("Invalid choice recieved.")
    }
}

fn cpuinfo(system: sysinfo::System) {
    println!("{} cores", system.get_processors().len());

    for cpu in system.get_processors() {
        let cpunum = cpu.get_name();
        let mut usage = cpu.get_cpu_usage() * 100.;
        usage = usage.round() / 100.;
        println!("{}: {}% usage", cpunum, usage)
    }
}

fn diskinfo(system: sysinfo::System) {
    for disk in system.get_disks() {
        let totalsize = disk.get_total_space() as f64;
        let freespace = disk.get_available_space() as f64;
        let usedspace = totalsize - freespace;
        let mut percent = (usedspace / totalsize) * 100.;
        percent = roundplaces(percent, 2);

        let ftotalsize = roundplaces(totalsize / 1e+9 as f64, 2);
        let ffreespace = roundplaces(freespace / 1e+9 as f64, 2);
        let fusedspace = roundplaces(usedspace / 1e+9 as f64, 2);
        // TODO: disk.get_name() returns "" for physical disks on windows

        println!("Disk: {:?}", disk.get_name());
        println!("--- Type: {:?}", disk.get_type());
        println!("--- Mountpoint: {:?}", disk.get_mount_point());
        println!("--- Total size: {} GB", ftotalsize );
        println!("--- Free space: {} GB", ffreespace );
        println!("--- Used space: {} GB", fusedspace );
        println!("--- Total usage: {}%", percent );
    }
}

fn meminfo(system: sysinfo::System) {
    let totalmemory = system.get_total_memory() as f64;
    let usedmemory = system.get_used_memory() as f64;
    let totalswap = system.get_total_swap() as f64;
    let usedswap = system.get_used_swap() as f64;

    println!("Total physical memory: {} GB", roundplaces(totalmemory / 1e+6, 2));
    println!("Used physical memory: {} GB", roundplaces(usedmemory / 1e+6, 2));

    if totalswap != 0. {
        println!("Total swap memory: {} GB", roundplaces(totalswap / 1e+6, 2));
        println!("Used swap memory: {} GB", roundplaces(usedswap / 1e+6, 2));
    }
}