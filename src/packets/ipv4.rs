use super::tcp::TcpPacket;

pub struct Ipv4Packet {
    pub version: u8,
    pub ihl: u8,
    pub dscp: u8,
    pub ecn: u8,
    pub total_length: u16,
    pub identification: u16,
    pub flags: u8,
    pub fragment_offset: u16,
    pub ttl: u8,
    pub protocol: u8,
    pub header_checksum: u16,
    pub source: [u8; 4],
    pub destination: [u8; 4],
    pub options: Vec<u8>,
    pub payload: IpPayload,
    pub header_length: u8,
    checksum_valid: bool,
}

pub enum IpPayload {
    Tcp(TcpPacket),
    //Udp(UdpPacket),
    //Icmp(IcmpPacket),
    Unknown(Vec<u8>),
}
impl Ipv4Packet{
    pub fn parse(packet: &[u8]) -> Option<Self> {
        if packet.len() < 20 {
            return None; // Not enough data for IPv4 header
        }
        let ihl = (packet[0] & 0x0f) as usize;
        let header_length = ihl * 4;

        if packet.len() < header_length {
            return None; // Not enough data for the specified header length
        }

        let header_checksum = ((packet[10] as u16) << 8) | packet[11] as u16;
        let calculated_checksum = crate::utils::calculate_checksum(&packet[0..header_length]);
        let checksum_valid = header_checksum == calculated_checksum;

        let protocol = packet[9];

        let source = [packet[12], packet[13], packet[14], packet[15]];            
        let destination = [packet[16], packet[17], packet[18], packet[19]];

        let payload = match protocol { //1 ICMP, 17 UDP, 6 TCP
            6 => IpPayload::Tcp(TcpPacket::parse(&packet[header_length..], &source, &destination)?),
            //17 => IpPayload::Udp(UdpPacket::parse(&packet[header_length..])?),
            //1 => IpPayload::Icmp(IcmpPacket::parse(&packet[header_length..])?),
            _ => IpPayload::Unknown(packet[header_length..].to_vec()),
        };  

            Some(Ipv4Packet{
            version: (packet[0] & 0xf0) >> 4,
            ihl: packet[0] & 0x0f,
            dscp: (packet[1] & 0xfc) >> 2,
            ecn: (packet[1] & 0x03),
            total_length: (((packet[2]) as u16) << 8) + (packet[3] as u16),
            identification: ((packet[4] as u16) << 8) + packet[5] as u16,
            flags: packet[6] & 0b11100000,
            fragment_offset: (((packet[6] & 0b00011111) as u16) << 8) + packet[7] as u16,
            ttl: packet[8],
            protocol,
            header_checksum: ((packet[10] as u16) << 8) + (packet[11] as u16),
            header_length: header_length as u8,
            source: [packet[12], packet[13], packet[14], packet[15]],            
            destination: [packet[16], packet[17], packet[18], packet[19]],
            options: packet[20..header_length].to_vec(),
            payload,
            checksum_valid,
        })
    }
}

use std::fmt;
use crate::utils::to_ip_string;
use crate::utils::to_hex_string;

impl fmt::Display for Ipv4Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IPv4 Packet:\n")?;
        write!(f, "Version: {}\n", self.version)?;
        write!(f, "IHL: {}\n", self.ihl)?;
        write!(f, "DSCP: {}\n", self.dscp)?;
        write!(f, "ECN: {}\n", self.ecn)?;
        write!(f, "Total Length: {}\n", self.total_length)?;
        write!(f, "Header Length: {}\n", self.header_length)?;
        write!(f, "Identification: {}\n", self.identification)?;
        write!(f, "Flags: {}\n", self.flags)?;
        write!(f, "Fragment Offset: {}\n", self.fragment_offset)?;
        write!(f, "TTL: {}\n", self.ttl)?;
        write!(f, "Protocol: {}\n", self.protocol)?;
        write!(f, "Header Checksum: 0x{:04x} (Valid: {})\n", self.header_checksum, self.checksum_valid)?;
        write!(f, "Source IP: {}\n", to_ip_string(&self.source))?;
        write!(f, "Destination IP: {}\n", to_ip_string(&self.destination))?;
        write!(f, "Options: {}\n", to_hex_string(&self.options))?;
        write!(f, "Payload: {}\n", self.payload)?;
        Ok(())
    }
}

impl fmt::Display for IpPayload{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IpPayload::Unknown(data) => write!(f, "Unknown payload: {}\n", to_hex_string(data)),
            IpPayload::Tcp(data) => write!(f, "{}\n", data),
        }
    }
}
