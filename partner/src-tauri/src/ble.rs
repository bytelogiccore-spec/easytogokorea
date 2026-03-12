use serde::{Deserialize, Serialize};

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

pub struct BleChat {
    pub pin: String,
    pub is_host: bool,
    pub connected: bool,
    pub messages: Vec<ChatMessage>,
}

impl BleChat {
    pub fn new() -> Self {
        Self {
            pin: String::new(),
            is_host: false,
            connected: false,
            messages: Vec::new(),
        }
    }

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

    // Render QR to a Vec<u8> grayscale image
    let img = code.render::<qrcode::render::unicode::Dense1x2>()
        .dark_color(qrcode::render::unicode::Dense1x2::Dark)
        .light_color(qrcode::render::unicode::Dense1x2::Light)
        .build();

    // For simplicity, render as SVG instead of PNG to avoid image crate issues
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
