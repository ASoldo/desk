#[macro_use]
extern crate prettytable;

use prettytable::row;
use prettytable::{Cell, Row, Table};
use sysinfo::{Components, Disks, Networks, System};

fn main() {
    // Initialize the system struct
    let mut sys = System::new_all();
    sys.refresh_all();

    // Create disk table
    let mut disk_table = Table::new();
    disk_table.add_row(row![
        "Disk Name",
        "Total Space (bytes)",
        "Available Space (bytes)"
    ]);
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        disk_table.add_row(row![
            format!("{:<25}", disk.name().to_string_lossy()),
            format!("{:<25}", disk.total_space()),
            format!("{:<25}", disk.available_space())
        ]);
    }

    // Create network table
    let mut network_table = Table::new();
    network_table.add_row(row![
        "Interface Name",
        "Total Received (bytes)",
        "Total Transmitted (bytes)"
    ]);
    let networks = Networks::new_with_refreshed_list();
    for (interface_name, data) in &networks {
        network_table.add_row(row![
            format!("{:<25}", interface_name),
            format!("{:<25}", data.total_received()),
            format!("{:<25}", data.total_transmitted())
        ]);
    }

    // Create component table
    let mut component_table = Table::new();
    component_table.add_row(row!["Component", "Temperature (C)"]);
    let components = Components::new_with_refreshed_list();
    for component in &components {
        component_table.add_row(row![
            format!("{:<39}", component.label()),
            format!("{:<39}", component.temperature())
        ]);
    }

    // Create system information table
    let mut sys_table = Table::new();
    sys_table.add_row(row!["Component", "Details"]);

    sys_table.add_row(row![
        "Total Memory (bytes)",
        format!("{:<45}", sys.total_memory())
    ]);
    sys_table.add_row(row![
        "Used Memory (bytes)",
        format!("{:<45}", sys.used_memory())
    ]);
    sys_table.add_row(row![
        "Total Swap (bytes)",
        format!("{:<45}", sys.total_swap())
    ]);
    sys_table.add_row(row![
        "Used Swap (bytes)",
        format!("{:<45}", sys.used_swap())
    ]);
    sys_table.add_row(row![
        "System Name",
        format!(
            "{:<45}",
            System::name().unwrap_or_else(|| "Unknown".to_string())
        )
    ]);
    sys_table.add_row(row![
        "Kernel Version",
        format!(
            "{:<45}",
            System::kernel_version().unwrap_or_else(|| "Unknown".to_string())
        )
    ]);
    sys_table.add_row(row![
        "OS Version",
        format!(
            "{:<45}",
            System::os_version().unwrap_or_else(|| "Unknown".to_string())
        )
    ]);
    sys_table.add_row(row![
        "Host Name",
        format!(
            "{:<45}",
            System::host_name().unwrap_or_else(|| "Unknown".to_string())
        )
    ]);
    sys_table.add_row(row!["Number of CPUs", format!("{:<45}", sys.cpus().len())]);

    // Adjust column widths to fill the section
    for row in sys_table.row_iter_mut() {
        row[0] = Cell::new(&format!("{:<33}", row[0].get_content()));
        row[1] = Cell::new(&format!("{:<33}", row[1].get_content()));
    }

    for row in component_table.row_iter_mut() {
        row[0] = Cell::new(&format!("{:<25}", row[0].get_content()));
        row[1] = Cell::new(&format!("{:<25}", row[1].get_content()));
    }

    for row in network_table.row_iter_mut() {
        row[0] = Cell::new(&format!("{:<25}", row[0].get_content()));
        row[1] = Cell::new(&format!("{:<25}", row[1].get_content()));
        row[2] = Cell::new(&format!("{:<25}", row[2].get_content()));
    }

    for row in disk_table.row_iter_mut() {
        row[0] = Cell::new(&format!("{:<25}", row[0].get_content()));
        row[1] = Cell::new(&format!("{:<25}", row[1].get_content()));
        row[2] = Cell::new(&format!("{:<25}", row[2].get_content()));
    }

    // Balance the number of rows in each table by adding empty rows
    while sys_table.len() < component_table.len() {
        sys_table.add_row(row!["", ""]);
    }
    while network_table.len() < disk_table.len() {
        network_table.add_row(row!["", "", ""]);
    }

    // Apply formatting to headers
    sys_table.set_titles(Row::new(vec![
        Cell::new("System Information").style_spec("bFg")
    ]));
    component_table.set_titles(Row::new(vec![
        Cell::new("Component Temperatures").style_spec("bFg")
    ]));
    network_table.set_titles(Row::new(vec![
        Cell::new("Network Interfaces").style_spec("bFg")
    ]));
    disk_table.set_titles(Row::new(vec![
        Cell::new("Disk Information").style_spec("bFg")
    ]));

    // Combine the tables into one main table
    let main_table = table!(
        [cbFg->"System Information", cbFg->"Component Temperatures"],
        [sys_table, component_table],
        [cbFg->"Network Interfaces", cbFg->"Disk Information"],
        [network_table, disk_table]
    );

    // Print the main table
    main_table.printstd();
}
