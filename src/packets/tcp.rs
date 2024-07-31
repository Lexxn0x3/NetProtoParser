use std::usize;

pub struct TcpPacket{
    pub source_port: u16,
    pub destination_port: u16,
    pub sequence_number: u32,
    pub ack_number:u32,
    pub data_offset: u8,
    pub reserved: u8,
    pub control_flags: u16,
    pub window_size: u16,
    pub checksum: u16,
    pub urgent_pointer: u16,
    pub optional_data: Vec<u8>,
    pub payload: TcpPayload,
    pub checksum_valid: bool,
}

#[derive(Debug)]
pub enum TcpPayload{
    Unknown(Vec<u8>),
}

impl TcpPacket{
   pub fn parse(packet: &[u8], src_ip: &[u8; 4], dest_ip: &[u8; 4]) -> Option<TcpPacket> {
        if packet.len() < 20 {
            return None; // Not enough data for basic TCP header
        }

        let data_offset = (packet[12] >> 4) * 4;
        if packet.len() < data_offset as usize {
            return None; // Not enough data for TCP header with options
        }

        let optional_data = packet[20..data_offset as usize].to_vec();

        let destination_port = ((packet[2] as u16) << 8) | (packet[3] as u16);
        let payload = match destination_port{ //1 ICMP, 17 UDP, 6 TCP
            //6 => IpPayload::Tcp(TcpPacket::parse(&packet[header_length..])?),
            //17 => IpPayload::Udp(UdpPacket::parse(&packet[header_length..])?),
            //1 => IpPayload::Icmp(IcmpPacket::parse(&packet[header_length..])?),
            _ => TcpPayload::Unknown(packet[data_offset as usize..].to_vec()),
        };

        
        let checksum = ((packet[16] as u16) << 8) | (packet[17] as u16);
        let checksum_valid = false;

        Some(TcpPacket{
            source_port: ((packet[0] as u16) << 8) | (packet[1] as u16),
            destination_port: ((packet[2] as u16) << 8) | (packet[3] as u16),
            sequence_number: ((packet[4] as u32) << 24) | ((packet[5] as u32) << 16) | ((packet[6] as u32) << 8) | (packet[7] as u32),
            ack_number: ((packet[8] as u32) << 24) | ((packet[9] as u32) << 16) | ((packet[10] as u32) << 8) | (packet[11] as u32),
            data_offset: packet[12] >> 4,  
            reserved: (packet[12] & 0b00001110) >> 1,
            control_flags: ((packet[12] & 0b00000001) as u16) << 8 | packet[13] as u16,
            window_size: ((packet[14] as u16) << 8) | (packet[15] as u16),
            checksum, 
            urgent_pointer: ((packet[18] as u16) << 8) | (packet[19] as u16),
            optional_data,
            payload,
            checksum_valid,
        })
    }
}


use std::fmt;
use crate::utils::to_hex_string;

impl fmt::Display for TcpPacket{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TCP Packet:\n")?;
        write!(f, "Source Port: {} (0x{:04x})\n", self.source_port, self.source_port)?;
        write!(f, "Destination Port: {} (0x{:04x})\n", self.destination_port, self.destination_port)?;
        write!(f, "Sequence Number: {} (0x{:08x})\n", self.sequence_number, self.sequence_number)?;
        write!(f, "Acknowledgment Number: {} (0x{:08x})\n", self.ack_number, self.ack_number)?;
        write!(f, "Data Offset: {}\n", self.data_offset)?;
        write!(f, "Reserved: 0x{:x}\n", self.reserved)?;
        write!(f, "Control Flags: 0x{:04x}\n", self.control_flags)?;
        write!(f, "Window Size: {} (0x{:04x})\n", self.window_size, self.window_size)?;
        write!(f, "Checksum: {} (0x{:04x}) (Valid: {})\n", self.checksum, self.checksum, self.checksum_valid)?;
        write!(f, "Urgent Pointer: {} (0x{:04x})\n", self.urgent_pointer, self.urgent_pointer)?;
        write!(f, "Optional Data: {}\n", to_hex_string(&self.optional_data))?;
        write!(f, "Payload: {:?}\n", self.payload)?;
        Ok(())
    }
}

impl fmt::Display for TcpPayload{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TcpPayload::Unknown(data) => write!(f, "Unknown payload: {}\n", to_hex_string(data)),
        }
    }
}
