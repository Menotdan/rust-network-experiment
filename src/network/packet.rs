use std::net::TcpStream;

use crate::core::game_state::GameState;
use super::serialization::Serialization;

pub trait Packet: Serialization {
    fn get_id(&self) -> u32;
    fn get_client_id(&self) -> u32;
    fn operate(&self, game_state: &mut GameState, source_stream: Box<TcpStream>) -> bool;
    fn set_client_id(&mut self, id: u32) -> ();
    fn get_new(&self) -> Box<dyn Packet>;
}