extern crate pnet;

use packet_hanler::to_hex_string;
use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};


mod packet_hanler;
mod ip_payload_handler;

mod packets;
use packets::ethernet::EthernetPacket as MyEthernetPacket;

mod utils;

use pnet::packet::Packet;
use utils::to_mac_string;

// Invoke as echo <interface name>
fn main() -> ! {
    let interface_name = "ens160"; 
    let interface_names_match =
        |iface: &NetworkInterface| iface.name == interface_name;

    // Find the network interface with the provided name
    let interfaces = datalink::interfaces();
    let interface = interfaces.into_iter()
                              .filter(interface_names_match)
                              .next()
                              .unwrap();

    // Create a new channel, dealing with layer 2 packets
    let (_tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("An error occurred when creating the datalink channel: {}", e)
    };
/*
    let ipv4_handler = packet_hanler::Ipv4PacketHandler; 
    let arp_handler = packet_hanler::ArpPacketHandler; 
    let dcp_handler = packet_hanler::DcpPacketHandler;
    let default_handler = DefaultPacketHandler;
*/
    loop {
        match rx.next() {
            Ok(packet) => {
                let packet = EthernetPacket::new(packet).unwrap();

                if let Some(my_packet) = MyEthernetPacket::parse(packet.packet()) {
                    if my_packet.ethertype == 2054 || true{
                    println!("{}", my_packet);
                    }
                } else {
                    println!("Failed to parse custom Ethernet packet");
                }
 /*
                if packet.get_ethertype().0 == 0x8892 {
                    dcp_handler.handle(&packet);
                }
                else{
                match packet.get_ethertype(){
                    EtherTypes::Ipv4 => ipv4_handler.handle(&packet),
                    EtherTypes::Arp => arp_handler.handle(&packet),
                    _ => default_handler.handle(&packet),
                }
                }
                //println!("Source:\t\t{}\nDestination:\t{}\nEthertype:\t{}\nPayload:\t{:?}",packet.get_source(), packet.get_destination(), packet.get_ethertype(), paylod);
*/
            },
            Err(e) => {
                // If an error occurs, we can handle it here
                panic!("An error occurred while reading: {}", e);
            }
        }
    }
}
