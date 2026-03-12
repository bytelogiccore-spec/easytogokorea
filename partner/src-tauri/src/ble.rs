use serde::{Deserialize, Serialize};
use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter, WriteType};
use btleplug::platform::{Adapter, Manager, Peripheral};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

// Custom EasyToGo BLE Chat service UUID
pub const CHAT_SERVICE_UUID: Uuid = Uuid::from_u128(0xEA570600_0001_4000_8000_00805F9B34FB);
pub const CHAT_MSG_CHAR_UUID: Uuid = Uuid::from_u128(0xEA570600_0002_4000_8000_00805F9B34FB);
pub const CHAT_NAME_CHAR_UUID: Uuid = Uuid::from_u128(0xEA570600_0003_4000_8000_00805F9B34FB);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChatStatus {
    pub connected: bool,
    pub is_host: bool,
    pub pin: String,
    pub peer_count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct NearbyDevice {
    pub name: String,
    pub id: String,
    pub rssi: i16,
}

/// Real BLE Chat implementation backed by btleplug
pub struct BleChat {
    pub pin: String,
    pub is_host: bool,
    pub connected: bool,
    pub messages: Vec<ChatMessage>,
    pub nearby_devices: Vec<NearbyDevice>,
    adapter: Option<Arc<Adapter>>,
    connected_peripheral: Option<Peripheral>,
}

impl BleChat {
    pub fn new() -> Self {
        Self {
            pin: String::new(),
            is_host: false,
            connected: false,
            messages: Vec::new(),
            nearby_devices: Vec::new(),
            adapter: None,
            connected_peripheral: None,
        }
    }

    /// Initialize the BLE adapter
    pub async fn init_adapter(&mut self) -> Result<(), String> {
        let manager = Manager::new().await.map_err(|e| format!("BLE Manager error: {e}"))?;
        let adapters = manager.adapters().await.map_err(|e| format!("Adapter error: {e}"))?;
        
        if let Some(adapter) = adapters.into_iter().next() {
            self.adapter = Some(Arc::new(adapter));
            Ok(())
        } else {
            Err("No BLE adapter found".to_string())
        }
    }

    /// Start scanning for nearby EasyToGo devices
    pub async fn start_scan(&mut self) -> Result<(), String> {
        let adapter = self.adapter.as_ref().ok_or("Adapter not initialized")?;
        
        adapter.start_scan(ScanFilter::default())
            .await
            .map_err(|e| format!("Scan error: {e}"))?;

        self.messages.push(ChatMessage {
            msg_type: "system".into(),
            text: "BLE 스캔 시작...".into(),
        });

        Ok(())
    }

    /// Stop scanning
    pub async fn stop_scan(&self) -> Result<(), String> {
        if let Some(adapter) = &self.adapter {
            adapter.stop_scan().await.map_err(|e| format!("Stop scan error: {e}"))?;
        }
        Ok(())
    }

    /// Get list of discovered peripherals
    pub async fn get_nearby_devices(&mut self) -> Result<Vec<NearbyDevice>, String> {
        let adapter = self.adapter.as_ref().ok_or("Adapter not initialized")?;
        let peripherals = adapter.peripherals().await
            .map_err(|e| format!("Peripherals error: {e}"))?;

        let mut devices = Vec::new();
        for peripheral in peripherals {
            let props = peripheral.properties().await
                .map_err(|e| format!("Properties error: {e}"))?;
            
            if let Some(props) = props {
                let name = props.local_name.unwrap_or_else(|| "Unknown".to_string());
                // Only show devices advertising our service or with EasyToGo in name
                if name.contains("EasyToGo") || name.contains("easytogo") {
                    devices.push(NearbyDevice {
                        name,
                        id: peripheral.id().to_string(),
                        rssi: props.rssi.unwrap_or(0),
                    });
                }
            }
        }

        self.nearby_devices = devices.clone();
        Ok(devices)
    }

    /// Connect to a specific peripheral by ID
    pub async fn connect_to_device(&mut self, device_id: &str) -> Result<(), String> {
        let adapter = self.adapter.as_ref().ok_or("Adapter not initialized")?;
        let peripherals = adapter.peripherals().await
            .map_err(|e| format!("Peripherals error: {e}"))?;

        for peripheral in peripherals {
            if peripheral.id().to_string() == device_id {
                peripheral.connect().await
                    .map_err(|e| format!("Connect error: {e}"))?;
                
                peripheral.discover_services().await
                    .map_err(|e| format!("Service discovery error: {e}"))?;

                self.connected = true;
                self.connected_peripheral = Some(peripheral);
                self.messages.push(ChatMessage {
                    msg_type: "system".into(),
                    text: format!("디바이스에 연결됨: {device_id}"),
                });
                return Ok(());
            }
        }

        Err(format!("Device {device_id} not found"))
    }

    /// Send a message via BLE characteristic write
    pub async fn send_ble_message(&mut self, text: &str) -> Result<(), String> {
        if let Some(peripheral) = &self.connected_peripheral {
            let chars = peripheral.characteristics();
            for char in &chars {
                if char.uuid == CHAT_MSG_CHAR_UUID {
                    peripheral.write(char, text.as_bytes(), WriteType::WithResponse)
                        .await
                        .map_err(|e| format!("Write error: {e}"))?;
                    
                    self.messages.push(ChatMessage {
                        msg_type: "sent".into(),
                        text: text.to_string(),
                    });
                    return Ok(());
                }
            }
            Err("Chat characteristic not found on device".to_string())
        } else {
            // Fallback: local message queue (for demo/testing)
            self.messages.push(ChatMessage {
                msg_type: "sent".into(),
                text: text.to_string(),
            });
            Ok(())
        }
    }

    // Legacy methods for backward compatibility
    pub fn start_host(&mut self, pin: String) {
        self.pin = pin;
        self.is_host = true;
        self.messages.push(ChatMessage {
            msg_type: "system".into(),
            text: format!("Room created with PIN: {}", self.pin),
        });
    }

    pub fn add_message(&mut self, msg: ChatMessage) {
        self.messages.push(msg);
    }

    pub fn status(&self) -> ChatStatus {
        ChatStatus {
            connected: self.connected,
            is_host: self.is_host,
            pin: self.pin.clone(),
            peer_count: if self.connected { 1 } else { 0 },
        }
    }

    pub fn drain_messages(&mut self) -> Vec<ChatMessage> {
        self.messages.drain(..).collect()
    }
}

pub fn generate_qr_base64(data: &str) -> String {
    use qrcode::QrCode;
    use base64::Engine;

    let code = match QrCode::new(data.as_bytes()) {
        Ok(c) => c,
        Err(_) => return String::new(),
    };

    let svg = render_qr_svg(&code);
    let b64 = base64::engine::general_purpose::STANDARD.encode(svg.as_bytes());
    format!("data:image/svg+xml;base64,{}", b64)
}

fn render_qr_svg(code: &qrcode::QrCode) -> String {
    let module_size = 8;
    let quiet = 4;
    let width = code.width();
    let total = (width + quiet * 2) * module_size;

    let mut svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {} {}" width="{}" height="{}">"#,
        total, total, total, total
    );
    svg.push_str(&format!(r#"<rect width="{}" height="{}" fill="white"/>"#, total, total));

    for y in 0..width {
        for x in 0..width {
            if code[(x, y)] == qrcode::types::Color::Dark {
                let px = (x + quiet) * module_size;
                let py = (y + quiet) * module_size;
                svg.push_str(&format!(
                    r#"<rect x="{}" y="{}" width="{}" height="{}" fill="black"/>"#,
                    px, py, module_size, module_size
                ));
            }
        }
    }
    svg.push_str("</svg>");
    svg
}
