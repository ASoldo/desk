![image](https://github.com/user-attachments/assets/938d648f-2bf2-43ad-8659-d4f1f73b556e)

# Deck

Deck is a Rust-based system information tool that displays comprehensive details about the system's memory, CPU, network interfaces, disk usage, and component temperatures. This application leverages the `sysinfo` and `prettytable` crates to present the information in a well-structured, nested table format.

## Features

- **System Information**: Displays total memory, used memory, total swap, used swap, system name, kernel version, OS version, host name, and number of CPUs.
- **Component Temperatures**: Lists the temperatures of various system components.
- **Network Interfaces**: Shows the network interface names along with total received and transmitted bytes.
- **Disk Information**: Provides details about each disk including its name, total space, and available space.

## Installation

1. **Clone the repository:**

   ```sh
   git clone https://github.com/ASoldo/desk.git
   ```

   ```sh

   cd desk
   ```

   Build the application:

```sh
cargo build --release

```

Run the application:

```sh
./target/release/desk

```

Upon running the application, Deck will display a nested table with the following sections:

- System Information: General details about the system.
- Component Temperatures: Current temperatures of various components.
- Network Interfaces: Information about network interfaces and data transfer statistics.
- Disk Information: Details about disk usage and available space.

## Dependencies

`sysinfo`: Provides access to system information.
`prettytable`: Used for creating and formatting tables.
