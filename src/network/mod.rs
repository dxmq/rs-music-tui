use crate::event::IoEvent;
use crate::network::network::Network;

pub(crate) mod cloud_music;
pub(crate) mod network;

#[tokio::main]
pub async fn start_tokio<'a>(io_rx: std::sync::mpsc::Receiver<IoEvent>, network: &mut Network) {
    while let Ok(io_event) = io_rx.recv() {
        network.handle_network_event(io_event).await;
    }
}
