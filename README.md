# NetProtoParser 🌐🔍

Welcome to **NetProtoParser**! This project is a powerful tool designed to parse and analyze various network protocols. With support for protocols like Ethernet, IPv4, ARP, TCP, and DCP, NetProtoParser makes it easy to inspect and debug network traffic.

## Features ✨

- **Ethernet Frame Parsing**: Extract and display Ethernet frame details.
- **IPv4 Packet Parsing**: Analyze IPv4 headers and payloads.
- **ARP Packet Parsing**: Inspect ARP requests and responses.
- **TCP Packet Parsing**: Dive into TCP headers and validate checksums.
- **DCP Packet Parsing**: Discover and configure devices using the Discovery and Configuration Protocol.
- **Checksum Verification**: Ensure data integrity with checksum validation.

## Getting Started 🚀

### Prerequisites

Ensure you have Rust installed. You can install Rust using [rustup](https://rustup.rs/).

### Installation

Clone the repository and navigate into the project directory:

```sh
git clone https://github.com/Lexxn0x3/NetProtoParser.git
cd NetProtoParser
```

Build the project using Cargo:
```sh
cargo build
```

## Example Output 📊

Here's an example of what the output might look like:

```
Ethernet Packet:
Destination: 00:0c:29:97:0d:68
Source: 00:0c:29:97:0d:67
Ethertype: 0x0800
Payload: IPv4 Packet:
    Source IP: 192.168.1.1
    Destination IP: 192.168.1.2
    ...

TCP Packet:
    Source Port: 443 (0x01bb)
    Destination Port: 34204 (0x859c)
    Sequence Number: 1523997736 (0x5ad65c28)
    Acknowledgment Number: 331941538 (0x13c906a2)
    Data Offset: 5
    Reserved: 0x0
    Control Flags: 0x0018
    Window Size: 64240 (0xfaf0)
    Checksum: 59750 (0xe966) (Valid: true)
    Urgent Pointer: 0 (0x0000)
    Optional Data: 
    Payload: Unknown

```

