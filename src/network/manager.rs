use crate::Result;
use crate::network::device::NetworkDevice;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{Duration, interval};

/// Network manager for handling network operations
pub struct NetworkManager {
    devices: Vec<Arc<Mutex<NetworkDevice>>>,
    running: bool,
}

impl NetworkManager {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
            running: false,
        }
    }

    /// Add a network device
    pub fn add_device(&mut self, device: NetworkDevice) -> Result<()> {
        let device = Arc::new(Mutex::new(device));
        self.devices.push(device);
        Ok(())
    }

    /// Start the network manager
    pub fn start(&mut self) -> Result<()> {
        self.running = true;
        
        // Start network processing tasks
        for device in &self.devices {
            let device = device.clone();
            tokio::spawn(async move {
                Self::network_processing_task(device).await;
            });
        }

        log::info!("Network manager started with {} devices", self.devices.len());
        Ok(())
    }

    /// Stop the network manager
    pub fn stop(&mut self) {
        self.running = false;
        log::info!("Network manager stopped");
    }

    /// Network processing task for a device
    async fn network_processing_task(device: Arc<Mutex<NetworkDevice>>) {
        let mut interval = interval(Duration::from_millis(10));
        
        loop {
            interval.tick().await;
            
            let mut device = device.lock().await;
            if !device.is_enabled() {
                continue;
            }

            // Process incoming packets
            if let Err(e) = device.process_incoming_packets().await {
                log::error!("Error processing incoming packets: {}", e);
            }

            // Process outgoing packets
            if let Err(e) = device.process_outgoing_packets().await {
                log::error!("Error processing outgoing packets: {}", e);
            }
        }
    }

    /// Send a packet through the first available device
    pub async fn send_packet(&self, packet: Vec<u8>) -> Result<()> {
        for device in &self.devices {
            let device = device.lock().await;
            if device.is_enabled() {
                return device.send_packet(packet).await;
            }
        }
        Err(crate::EmulatorError::Network("No enabled network devices".to_string()))
    }

    /// Receive a packet from any device
    pub async fn receive_packet(&self) -> Option<Vec<u8>> {
        for device in &self.devices {
            let device = device.lock().await;
            if device.is_enabled() {
                if let Some(packet) = device.receive_packet().await {
                    return Some(packet);
                }
            }
        }
        None
    }

    /// Get the number of network devices
    pub fn device_count(&self) -> usize {
        self.devices.len()
    }

    /// Check if the network manager is running
    pub fn is_running(&self) -> bool {
        self.running
    }
}
