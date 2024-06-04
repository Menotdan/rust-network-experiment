use std::{collections::HashMap, net::TcpStream};

use crate::types::pos::Pos;

pub struct Client {
    pub id: u32,
    pub position: Pos,
    pub stream: Box<TcpStream>
}

#[derive(Default)]
pub struct GameState {
    pub clients: HashMap<u32, Client>,
}