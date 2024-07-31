use pnet::packet::{ethernet::EthernetPacket, Packet};
extern crate pnet;

pub trait PacketHandler{
   fn handle(&self, packet: &EthernetPacket);
}

pub struct Ipv4PacketHandler;

impl PacketHandler for Ipv4PacketHandler {
    fn handle(&self, packet: &EthernetPacket) {
        let payload = packet.payload();

        // Assuming IPv4 addresses are at specific offsets within the payload
        if payload.len() >= 20 {
            let ihl = (payload[0] & 0x0f) as usize;
            let header_length = ihl*4;

            if payload.len() < header_length {
                print!("Invalid IPv4 packer: header len exceeds packet len");
                return;
            }
            let source_ip = ipv4_to_string(&payload[12..16]);
            let dest_ip = ipv4_to_string(&payload[16..20]);

            println!("Source IP: {}", source_ip);
            println!("Dest IP: {}", dest_ip);
            println!("Header length: {} bytes", header_length);
            println!("Protocol: {}",packet.payload()[9]);
            println!("IP Payload: {}", to_hex_string(&packet.payload()[header_length..]));
            println!()
        } else {
            println!("Payload too short to extract IP addresses");
        }

        // Print the full packet, including header and payload
        //println!("Full Ethernet Frame (in hex): {}", to_hex_string(packet.payload()));

    }
}

pub struct ArpPacketHandler;

impl PacketHandler for ArpPacketHandler{
    fn handle(&self, _packet: &EthernetPacket){
        println!("{:?}", &_packet.payload()[12..31]);
    }
}

pub struct DcpPacketHandler;

impl PacketHandler for DcpPacketHandler {
    fn handle(&self, packet: &EthernetPacket) {
        let payload = packet.payload();
        
        if payload.len() >= 8 {
            let service_id = payload[0];
            let service_type = payload[1];
            let xid = u16::from_be_bytes([payload[2], payload[3]]);
            let response_delay = u16::from_be_bytes([payload[4], payload[5]]);
            let dcp_data_length = u16::from_be_bytes([payload[6], payload[7]]);

            println!("DCP Packet:");
            println!("  Service ID: {}", service_id);
            println!("  Service Type: {}", service_type);
            println!("  XID: {}", xid);
            println!("  Response Delay: {}", response_delay);
            println!("  DCP Data Length: {}", dcp_data_length);
            println!("  DCP Data (hex): {}", to_hex_string(&payload[8..(8 + dcp_data_length as usize)]));
        } else {
            println!("DCP Packet payload too short to parse");
        }
    }
}

pub struct DefaultPacketHandler;

impl PacketHandler for DefaultPacketHandler{
    fn handle(&self, _packet: &EthernetPacket) {
        println!("Default packet");
    }
}
// Function to convert a byte slice to a hex string
pub fn to_hex_string(bytes: &[u8]) -> String {
    bytes.iter()
         .map(|b| format!("{:02x}", b))
         .collect::<Vec<String>>()
         .join(" ")
}
// Function to convert a byte slice representing an IPv4 address to a string
fn ipv4_to_string(bytes: &[u8]) -> String {
    format!("{}.{}.{}.{}", bytes[0], bytes[1], bytes[2], bytes[3])
}
