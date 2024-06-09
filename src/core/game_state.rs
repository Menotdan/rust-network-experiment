use std::{collections::HashMap, net::TcpStream};

use crate::types::pos::Pos;

pub struct Client {
    pub id: u32,
    pub position: Pos,
    pub stream: Option<Box<TcpStream>>,
    pub disconnected: bool
}

#[derive(Default)]
pub struct GameStateData {
    meow: u32,
}

#[derive(Default)]
pub struct GameState {
    pub clients: HashMap<u32, Client>,
    pub data: GameStateData,
}