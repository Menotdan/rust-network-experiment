use std::{io::Cursor, net::TcpStream};

use crate::{core::game_state::GameState, network::{packet::Packet, serialization::Serialization}, types::pos::Pos};

#[derive(Default)]
pub struct MovePacket {
    client_id: u32,
    from: Pos,
    to: Pos
}

impl Serialization for MovePacket {
    fn serialize(&self) -> Vec<u8> {        
        let mut from_bytes = self.from.serialize();
        let mut to_bytes = self.to.serialize();

        let mut out: Vec<u8> = Vec::new();
        out.append(&mut from_bytes);
        out.append(&mut to_bytes);

        return out;
    }

    fn deserialize(&mut self, data: &mut std::io::Cursor<Vec<u8>>) -> () {
        self.from.deserialize(data);
        self.to.deserialize(data);
    }
}

impl Packet for MovePacket {
    fn get_id(&self) -> u32 {
        return 1;
    }
    
    fn get_client_id(&self) -> u32 {
        return self.client_id;
    }

    fn set_client_id(&mut self, id: u32) -> () {
        self.client_id = id;
    }
    
    fn operate(&self, game_state: &mut GameState, _: Box<TcpStream>) -> Result<Box<dyn Packet>, bool>{
        let target = game_state.clients.get(&self.client_id);
        if target.is_none() {
            return Err(true);
        }

        let mut output_packet = self.get_new();
        output_packet.set_client_id(self.client_id);
        output_packet.deserialize(&mut Cursor::new(self.serialize()));

        return Ok(output_packet);
    }
    
    fn get_new(&self) -> Box<dyn Packet> {
        return Box::new(MovePacket::default());
    }
}