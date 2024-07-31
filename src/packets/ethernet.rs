use crate::utils::to_mac_string;
use crate::utils::to_hex_string;

use super::ipv4::Ipv4Packet;
use super::arp::ArpPacket;

pub struct EthernetPacket {
    pub destination: [u8; 6],
    pub source: [u8; 6],
    pub ethertype: u16,
    pub payload: EthernetPayload,
}

pub enum EthernetPayload {
    Ipv4(Ipv4Packet),
    ARP(ArpPacket),
    Unknown(Vec<u8>),
}

impl EthernetPacket {
    pub fn parse(packet: &[u8]) -> Option<Self> {
        if packet.len() < 14 {
            return None; // Not enough data for Ethernet header
        }
        let ethertype = ((packet[12] as u16) << 8) | packet[13] as u16;
        let payload = match ethertype {
            0x0800 => EthernetPayload::Ipv4(Ipv4Packet::parse(&packet[14..])?),
            2054 => EthernetPayload::ARP(ArpPacket::parse(&packet[14..])?),
            _ => EthernetPayload::Unknown(packet[14..].to_vec()),
        };
        Some(EthernetPacket {
            destination: [packet[0], packet[1], packet[2], packet[3], packet[4], packet[5]],
            source: [packet[6], packet[7], packet[8], packet[9], packet[10], packet[11]],
            ethertype,
            payload,
        })
    }
}

use std::fmt;

impl fmt::Display for EthernetPacket{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "Destination: {}\n", to_mac_string(&self.destination))?;
        write!(f, "Source: {}\n", to_mac_string(&self.source))?;
        write!(f, "Ethertype: {}\n", self.ethertype)?;
        write!(f, "{}\n", self.payload)?;
        Ok(())
    }
}

impl fmt::Display for EthernetPayload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EthernetPayload::Ipv4(packet) => write!(f, "{}", packet),
            EthernetPayload::ARP(packet) => write!(f, "{}", packet),
            EthernetPayload::Unknown(data) => write!(f, "Unknown payload: {}\n", to_hex_string(data)),
        }
    }
}
