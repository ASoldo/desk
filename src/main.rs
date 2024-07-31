#[macro_use]
extern crate prettytable;

use prettytable::row;
use prettytable::Table;
use sysinfo::{Components, Disks, Networks, System};

fn main() {
    // Please note that we use "new_all" to ensure that all lists of
    // CPUs and processes are filled!
    let mut sys = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();

    let mut disk_table = Table::new();
    disk_table.add_row(
        row![cbFg->"Disk Name", cbFg->"Total Space (bytes)", cbFg->"Available Space (bytes)"],
    );
    // We display all disks' information:
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        disk_table.add_row(row![
            disk.name().to_string_lossy(),
            disk.total_space(),
            disk.available_space()
        ]);
    }

    // Network interfaces name, total data received and total data transmitted:
    let networks = Networks::new_with_refreshed_list();
    let mut network_table = Table::new();
    network_table.add_row(row![
        cbFg->"Interface Name",
        cbFg->"Total Received (bytes)",
        cbFg->"Total Transmitted (bytes)"
    ]);
    for (interface_name, data) in &networks {
        network_table.add_row(row![
            interface_name,
            data.total_received(),
            data.total_transmitted()
        ]);
    }

    // Components temperature:
    let components = Components::new_with_refreshed_list();
    let mut component_table = Table::new();
    component_table.add_row(row![cbFg->"Component", cbFg->"Temperature (C)"]);
    for component in &components {
        component_table.add_row(row![component.label(), component.temperature()]);
    }

    // Create a new table for system information
    let mut sys_table = Table::new();
    sys_table.add_row(row![cbFg->"Component", cbFg->"Details"]);

    sys_table.add_row(row!["Total Memory (bytes)", sys.total_memory()]);
    sys_table.add_row(row!["Used Memory (bytes)", sys.used_memory()]);
    sys_table.add_row(row!["Total Swap (bytes)", sys.total_swap()]);
    sys_table.add_row(row!["Used Swap (bytes)", sys.used_swap()]);
    sys_table.add_row(row![
        "System Name",
        System::name().unwrap_or_else(|| "Unknown".to_string())
    ]);
    sys_table.add_row(row![
        "Kernel Version",
        System::kernel_version().unwrap_or_else(|| "Unknown".to_string())
    ]);
    sys_table.add_row(row![
        "OS Version",
        System::os_version().unwrap_or_else(|| "Unknown".to_string())
    ]);
    sys_table.add_row(row![
        "Host Name",
        System::host_name().unwrap_or_else(|| "Unknown".to_string())
    ]);
    sys_table.add_row(row!["Number of CPUs", sys.cpus().len()]);

    // Combine the tables into one main table
    let main_table = table!(
        [cbFg->"System Information", cbFg->"Component Temperatures"],
        [sys_table, component_table],
        [cbFg->"Network Interfaces", cbFg->"Disk Information"],
        [network_table, disk_table]
    );

    main_table.printstd();
}
