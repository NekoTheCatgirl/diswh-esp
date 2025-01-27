pub use color::*;
pub use edit::*;
pub use edit_builder::*;
pub use embed::*;
pub use embed_builder::*;
pub use message::*;
pub use message_builder::*;

pub mod color;
pub mod edit;
pub mod edit_builder;
pub mod embed;
pub mod embed_builder;
pub mod message;
pub mod message_builder;

use log::{error, info};

use embedded_svc::{http::client::Client as HttpClient, io::Write, utils::io};
use esp_idf_svc::http::{client::{Configuration, EspHttpConnection}, Method};

#[derive(Clone)]
pub struct WebhookBuilder {
    url: String,
}

impl WebhookBuilder {
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }

    pub fn send_message(self, packet: MessagePacket) -> anyhow::Result<Self> {
        WebhookBuilder::send_packet(&self.url.clone(), false, &packet.serialize_packet())?;
        Ok(self)
    }

    pub fn edit_message(self, packet: EditMessagePacket, id: usize) -> anyhow::Result<Self> {
        WebhookBuilder::send_packet(
            &(self.url.clone() + &format!("/messages/{}", id)),
            true,
            &packet.serialize_packet(),
        )?;
        Ok(self)
    }

    fn send_packet(url: &str, patch: bool, packet: &str) -> anyhow::Result<()> {
        let mut client = HttpClient::wrap(EspHttpConnection::new(
            &Configuration {
                use_global_ca_store: true,
                crt_bundle_attach: Some(esp_idf_svc::sys::esp_crt_bundle_attach),
                ..Default::default()
            }
        )?);


        let headers = [("Content-Type", "application/json")];
        let method = if patch { Method::Patch } else { Method::Post };

        let mut request = client.request(method, url, &headers)?;
        request.write_all(packet.as_bytes())?;
        request.flush()?;
        if patch {
            info!("-> PATCH {}", url);
        } else {
            info!("-> POST {}", url);
        }
        let mut response = request.submit()?;

        // Process response
        let status = response.status();
        info!("<- {}", status);
        let mut buf = [0u8; 1024];
        let bytes_read = io::try_read_full(&mut response, &mut buf).map_err(|e| e.0)?;
        info!("Read {} bytes", bytes_read);
        match std::str::from_utf8(&buf[0..bytes_read]) {
            Ok(body_string) => info!(
                "Response body (truncated to {} bytes): {:?}",
                buf.len(),
                body_string
            ),
            Err(e) => error!("Error decoding response body: {}", e),
        };

        client.release();

        Ok(())
    }
}
