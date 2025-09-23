use crate::Result;
use pnet::datalink::{self, NetworkInterface};
use pnet::packet::{ethernet::EthernetPacket, ipv4::Ipv4Packet, tcp::TcpPacket, udp::UdpPacket};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::VecDeque;

/// Network device for the virtual machine
pub struct NetworkDevice {
    interface: NetworkInterface,
    rx_queue: Arc<Mutex<VecDeque<Vec<u8>>>>,
    tx_queue: Arc<Mutex<VecDeque<Vec<u8>>>>,
    mac_address: [u8; 6],
    ip_address: [u8; 4],
    enabled: bool,
}

impl NetworkDevice {
    pub fn new(interface_name: &str) -> Result<Self> {
        let interfaces = datalink::interfaces();
        let interface = interfaces
            .into_iter()
            .find(|iface| iface.name == interface_name)
            .ok_or_else(|| crate::EmulatorError::Network(format!("Interface {} not found", interface_name)))?;

        let mac_address = interface.mac.unwrap_or_else(|| pnet::datalink::MacAddr::new(0x02, 0x00, 0x00, 0x00, 0x00, 0x01));
        let ip_address = interface.ips
            .iter()
            .find(|ip| ip.is_ipv4())
            .map(|ip| match ip.ip() {
                std::net::IpAddr::V4(ipv4) => ipv4.octets(),
                _ => [192, 168, 1, 100], // Default IP
            })
            .unwrap_or([192, 168, 1, 100]);

        Ok(Self {
            interface,
            rx_queue: Arc::new(Mutex::new(VecDeque::new())),
            tx_queue: Arc::new(Mutex::new(VecDeque::new())),
            mac_address: mac_address.into(),
            ip_address,
            enabled: false,
        })
    }

    pub fn enable(&mut self) -> Result<()> {
        self.enabled = true;
        log::info!("Network device enabled on interface {}", self.interface.name);
        Ok(())
    }

    pub fn disable(&mut self) {
        self.enabled = false;
        log::info!("Network device disabled");
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn get_mac_address(&self) -> [u8; 6] {
        self.mac_address
    }

    pub fn get_ip_address(&self) -> [u8; 4] {
        self.ip_address
    }

    /// Send a packet
    pub async fn send_packet(&self, packet: Vec<u8>) -> Result<()> {
        if !self.enabled {
            return Err(crate::EmulatorError::Network("Network device not enabled".to_string()));
        }

        let mut tx_queue = self.tx_queue.lock().await;
        tx_queue.push_back(packet);
        Ok(())
    }

    /// Receive a packet
    pub async fn receive_packet(&self) -> Option<Vec<u8>> {
        let mut rx_queue = self.rx_queue.lock().await;
        rx_queue.pop_front()
    }

    /// Process incoming packets from the host network
    pub async fn process_incoming_packets(&self) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        // In a real implementation, this would read from the network interface
        // For now, we'll simulate some network activity
        Ok(())
    }

    /// Process outgoing packets to the host network
    pub async fn process_outgoing_packets(&self) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        let mut tx_queue = self.tx_queue.lock().await;
        while let Some(packet) = tx_queue.pop_front() {
            // In a real implementation, this would send the packet to the network interface
            log::debug!("Sending packet of {} bytes", packet.len());
        }
        Ok(())
    }

    /// Create an Ethernet frame
    pub fn create_ethernet_frame(&self, dest_mac: [u8; 6], ethertype: u16, payload: &[u8]) -> Vec<u8> {
        let mut frame = Vec::new();
        
        // Destination MAC
        frame.extend_from_slice(&dest_mac);
        
        // Source MAC
        frame.extend_from_slice(&self.mac_address);
        
        // EtherType
        frame.extend_from_slice(&ethertype.to_be_bytes());
        
        // Payload
        frame.extend_from_slice(payload);
        
        frame
    }

    /// Parse an Ethernet frame
    pub fn parse_ethernet_frame(&self, frame: &[u8]) -> Result<EthernetFrame> {
        if frame.len() < 14 {
            return Err(crate::EmulatorError::Network("Frame too short".to_string()));
        }

        let dest_mac = [frame[0], frame[1], frame[2], frame[3], frame[4], frame[5]];
        let src_mac = [frame[6], frame[7], frame[8], frame[9], frame[10], frame[11]];
        let ethertype = u16::from_be_bytes([frame[12], frame[13]]);
        let payload = frame[14..].to_vec();

        Ok(EthernetFrame {
            dest_mac,
            src_mac,
            ethertype,
            payload,
        })
    }

    /// Create an ARP request
    pub fn create_arp_request(&self, target_ip: [u8; 4]) -> Vec<u8> {
        let mut arp_packet = Vec::new();
        
        // Hardware type (Ethernet)
        arp_packet.extend_from_slice(&1u16.to_be_bytes());
        
        // Protocol type (IPv4)
        arp_packet.extend_from_slice(&0x0800u16.to_be_bytes());
        
        // Hardware address length
        arp_packet.push(6);
        
        // Protocol address length
        arp_packet.push(4);
        
        // Operation (ARP request)
        arp_packet.extend_from_slice(&1u16.to_be_bytes());
        
        // Sender hardware address
        arp_packet.extend_from_slice(&self.mac_address);
        
        // Sender protocol address
        arp_packet.extend_from_slice(&self.ip_address);
        
        // Target hardware address (unknown)
        arp_packet.extend_from_slice(&[0, 0, 0, 0, 0, 0]);
        
        // Target protocol address
        arp_packet.extend_from_slice(&target_ip);
        
        // Create Ethernet frame
        self.create_ethernet_frame([0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF], 0x0806, &arp_packet)
    }

    /// Create an ICMP ping packet
    pub fn create_icmp_ping(&self, dest_ip: [u8; 4], identifier: u16, sequence: u16) -> Vec<u8> {
        let mut icmp_packet = Vec::new();
        
        // ICMP type (Echo Request)
        icmp_packet.push(8);
        
        // ICMP code
        icmp_packet.push(0);
        
        // Checksum (will be calculated)
        icmp_packet.extend_from_slice(&0u16.to_be_bytes());
        
        // Identifier
        icmp_packet.extend_from_slice(&identifier.to_be_bytes());
        
        // Sequence number
        icmp_packet.extend_from_slice(&sequence.to_be_bytes());
        
        // Data (timestamp)
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        icmp_packet.extend_from_slice(&timestamp.to_be_bytes());
        
        // Calculate checksum
        let checksum = self.calculate_icmp_checksum(&icmp_packet);
        icmp_packet[2..4].copy_from_slice(&checksum.to_be_bytes());
        
        // Create IP packet
        let ip_packet = self.create_ipv4_packet(dest_ip, 1, &icmp_packet);
        
        // Create Ethernet frame
        self.create_ethernet_frame([0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF], 0x0800, &ip_packet)
    }

    fn create_ipv4_packet(&self, dest_ip: [u8; 4], protocol: u8, payload: &[u8]) -> Vec<u8> {
        let mut ip_packet = Vec::new();
        
        // Version and IHL
        ip_packet.push(0x45);
        
        // Type of Service
        ip_packet.push(0);
        
        // Total Length
        let total_length = (20 + payload.len()) as u16;
        ip_packet.extend_from_slice(&total_length.to_be_bytes());
        
        // Identification
        ip_packet.extend_from_slice(&0u16.to_be_bytes());
        
        // Flags and Fragment Offset
        ip_packet.extend_from_slice(&0u16.to_be_bytes());
        
        // Time to Live
        ip_packet.push(64);
        
        // Protocol
        ip_packet.push(protocol);
        
        // Header Checksum (will be calculated)
        ip_packet.extend_from_slice(&0u16.to_be_bytes());
        
        // Source IP
        ip_packet.extend_from_slice(&self.ip_address);
        
        // Destination IP
        ip_packet.extend_from_slice(&dest_ip);
        
        // Payload
        ip_packet.extend_from_slice(payload);
        
        // Calculate header checksum
        let checksum = self.calculate_ip_checksum(&ip_packet[0..20]);
        ip_packet[10..12].copy_from_slice(&checksum.to_be_bytes());
        
        ip_packet
    }

    fn calculate_icmp_checksum(&self, packet: &[u8]) -> u16 {
        let mut sum = 0u32;
        
        for chunk in packet.chunks(2) {
            if chunk.len() == 2 {
                sum += u16::from_be_bytes([chunk[0], chunk[1]]) as u32;
            } else {
                sum += (chunk[0] as u32) << 8;
            }
        }
        
        while (sum >> 16) != 0 {
            sum = (sum & 0xFFFF) + (sum >> 16);
        }
        
        !sum as u16
    }

    fn calculate_ip_checksum(&self, header: &[u8]) -> u16 {
        let mut sum = 0u32;
        
        for chunk in header.chunks(2) {
            if chunk.len() == 2 {
                sum += u16::from_be_bytes([chunk[0], chunk[1]]) as u32;
            } else {
                sum += (chunk[0] as u32) << 8;
            }
        }
        
        while (sum >> 16) != 0 {
            sum = (sum & 0xFFFF) + (sum >> 16);
        }
        
        !sum as u16
    }
}

/// Ethernet frame structure
#[derive(Debug, Clone)]
pub struct EthernetFrame {
    pub dest_mac: [u8; 6],
    pub src_mac: [u8; 6],
    pub ethertype: u16,
    pub payload: Vec<u8>,
}
