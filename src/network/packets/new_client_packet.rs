use std::net::TcpStream;

use crate::{core::game_state::{Client, GameState}, network::{packet::Packet, serialization::Serialization}, types::pos::Pos};

#[derive(Default)]
pub struct NewClientPacket {
    client_id: u32,
}

impl Serialization for NewClientPacket {
    fn serialize(&self) -> Vec<u8> {
        let out: Vec<u8> = Vec::new();

        return out;
    }

    fn deserialize(&mut self, _data: &mut std::io::Cursor<Vec<u8>>) -> () {
    }
}

impl Packet for NewClientPacket {
    fn get_id(&self) -> u32 {
        return 0;
    }
    
    fn get_client_id(&self) -> u32 {
        return self.client_id;
    }

    fn set_client_id(&mut self, id: u32) -> () {
        self.client_id = id;
    }
    
    fn operate(&self, game_state: &mut GameState, client_stream: Box<TcpStream>) -> bool {
        let mut i = 0;
        loop {
            if !game_state.clients.contains_key(&i) {
                break;
            }
            i += 1;
        }

        let new_client = Client { id: i, position: Pos::default(), stream: client_stream };
        game_state.clients.insert(i, new_client);
        return true;
    }

    fn get_new(&self) -> Box<dyn Packet> {
        return Box::new(NewClientPacket::default());
    }
}