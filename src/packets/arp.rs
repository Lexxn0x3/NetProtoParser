pub struct ArpPacket {
    pub hardware_type: u16,
    pub protocol_type: u16,
    pub hardware_size: u8,
    pub protocol_size: u8,
    pub opcode: u16,
    pub sender_mac: [u8; 6],
    pub sender_ip: [u8; 4],
    pub target_mac: [u8; 6],
    pub target_ip: [u8; 4],
}

impl ArpPacket {
    pub fn parse(packet: &[u8]) -> Option<Self> {
        if packet.len() < 28 {
            return None; // Not enough data for ARP packet
        }
        Some(ArpPacket {
            hardware_type: ((packet[0] as u16) << 8) | packet[1] as u16,
            protocol_type: ((packet[2] as u16) << 8) | packet[3] as u16,
            hardware_size: packet[4],
            protocol_size: packet[5],
            opcode: ((packet[6] as u16) << 8) | packet[7] as u16,
            sender_mac: [packet[8], packet[9], packet[10], packet[11], packet[12], packet[13]],
            sender_ip: [packet[14], packet[15], packet[16], packet[17]],
            target_mac: [packet[18], packet[19], packet[20], packet[21], packet[22], packet[23]],
            target_ip: [packet[24], packet[25], packet[26], packet[27]],
        })
    }
}

use std::fmt;
use crate::utils::to_mac_string;
use crate::utils::to_ip_string;

impl fmt::Display for ArpPacket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ARP Packet:\n")?;
        write!(f, "Hardware Type: {}\n", self.hardware_type)?;
        write!(f, "Protocol Type: {}\n", self.protocol_type)?;
        write!(f, "Hardware Size: {}\n", self.hardware_size)?;
        write!(f, "Protocol Size: {}\n", self.protocol_size)?;
        write!(f, "Opcode: {}\n", self.opcode)?;
        write!(f, "Sender MAC: {}\n", to_mac_string(&self.sender_mac))?;
        write!(f, "Sender IP: {}\n", to_ip_string(&self.sender_ip))?;
        write!(f, "Target MAC: {}\n", to_mac_string(&self.target_mac))?;
        write!(f, "Target IP: {}\n", to_ip_string(&self.target_ip))?;
        Ok(())
    }
}
