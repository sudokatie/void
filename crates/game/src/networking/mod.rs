//! Client-side networking for multiplayer.

pub mod commands;

use std::time::Duration;

use engine_network::{
    ClientMessage, GameClient, ServerMessage, WorldSnapshot,
    protocol::InputState,
};
use glam::Vec3;
use tracing::{debug, info, warn};

/// Client network state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkState {
    Disconnected,
    Connecting,
    Connected,
}

/// Server info received on connect.
#[derive(Clone, Debug)]
pub struct ServerInfo {
    pub player_id: u64,
    pub tick_rate: u32,
    pub seed: u64,
}

/// Network client for game.
pub struct NetworkClient {
    client: Option<GameClient>,
    state: NetworkState,
    server_info: Option<ServerInfo>,
    input_sequence: u32,
    last_snapshot: Option<WorldSnapshot>,
    pending_events: Vec<NetworkEvent>,
}

#[derive(Clone, Debug)]
pub enum NetworkEvent {
    Connected(ServerInfo),
    Disconnected,
    PlayerJoined { id: u64, name: String },
    PlayerLeft { id: u64 },
    ChatReceived { sender: String, message: String },
    BlockChanged { pos: (i32, i32, i32), block: u16 },
}

impl NetworkClient {
    pub fn new() -> Self {
        Self {
            client: None,
            state: NetworkState::Disconnected,
            server_info: None,
            input_sequence: 0,
            last_snapshot: None,
            pending_events: Vec::new(),
        }
    }

    pub fn connect(&mut self, address: &str) -> Result<(), String> {
        match GameClient::connect(address) {
            Ok(client) => {
                self.client = Some(client);
                self.state = NetworkState::Connecting;
                info!("Connecting to {address}");
                Ok(())
            }
            Err(e) => {
                warn!("Failed to connect: {e}");
                Err(e.to_string())
            }
        }
    }

    pub fn disconnect(&mut self) {
        if let Some(ref mut client) = self.client {
            client.disconnect();
        }
        self.client = None;
        self.state = NetworkState::Disconnected;
        self.server_info = None;
        self.last_snapshot = None;
        self.pending_events.push(NetworkEvent::Disconnected);
    }

    pub fn update(&mut self, dt: Duration) {
        let Some(ref mut client) = self.client else { return };
        client.update(dt);
        if !client.is_connected() && !client.is_connecting() {
            if self.state != NetworkState::Disconnected {
                self.state = NetworkState::Disconnected;
                self.pending_events.push(NetworkEvent::Disconnected);
            }
        }
        let messages: Vec<_> = client.receive();
        client.send_packets();
        for message in messages {
            self.process_message(message);
        }
    }

    fn process_message(&mut self, message: ServerMessage) {
        match message {
            ServerMessage::Welcome { player_id, tick_rate, seed } => {
                let info = ServerInfo { player_id, tick_rate, seed };
                self.server_info = Some(info.clone());
                self.state = NetworkState::Connected;
                self.pending_events.push(NetworkEvent::Connected(info));
            }
            ServerMessage::Snapshot(snapshot) => { self.last_snapshot = Some(snapshot); }
            ServerMessage::PlayerJoined { id, name } => {
                self.pending_events.push(NetworkEvent::PlayerJoined { id, name });
            }
            ServerMessage::PlayerLeft { id } => {
                self.pending_events.push(NetworkEvent::PlayerLeft { id });
            }
            ServerMessage::ChatReceive { sender, message } => {
                self.pending_events.push(NetworkEvent::ChatReceived { sender, message });
            }
            ServerMessage::BlockChange { pos, block } => {
                self.pending_events.push(NetworkEvent::BlockChanged { pos: (pos.0.x, pos.0.y, pos.0.z), block });
            }
            _ => { debug!("Received other message"); }
        }
    }

    pub fn send_input(&mut self, movement: Vec3, jump: bool, sprint: bool, yaw: f32, pitch: f32) {
        let Some(ref mut client) = self.client else { return };
        if !client.is_connected() { return; }
        self.input_sequence = self.input_sequence.wrapping_add(1);
        let input = InputState { movement, jump, sprint, yaw, pitch, sequence: self.input_sequence };
        if let Err(e) = client.send(&ClientMessage::Input(input)) { debug!("Failed to send input: {e}"); }
    }

    pub fn send_chat(&mut self, message: String) {
        let Some(ref mut client) = self.client else { return };
        if let Err(e) = client.send(&ClientMessage::ChatSend { message }) { warn!("Failed to send chat: {e}"); }
    }

    pub fn state(&self) -> NetworkState { self.state }
    pub fn is_connected(&self) -> bool { self.state == NetworkState::Connected }
    pub fn server_info(&self) -> Option<&ServerInfo> { self.server_info.as_ref() }
    pub fn last_snapshot(&self) -> Option<&WorldSnapshot> { self.last_snapshot.as_ref() }
    pub fn drain_events(&mut self) -> Vec<NetworkEvent> { std::mem::take(&mut self.pending_events) }
    pub fn rtt_ms(&self) -> f64 { self.client.as_ref().map_or(0.0, |c| c.rtt_ms()) }
}

impl Default for NetworkClient {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn network_client_initial_state() {
        let client = NetworkClient::new();
        assert_eq!(client.state(), NetworkState::Disconnected);
        assert!(!client.is_connected());
    }
}
