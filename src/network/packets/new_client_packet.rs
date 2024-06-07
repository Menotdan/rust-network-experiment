use std::net::TcpStream;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::{core::game_state::{Client, GameState}, network::{packet::Packet, serialization::Serialization}, types::pos::Pos};

#[derive(Default)]
pub struct NewClientPacket {
    client_id: u32,
    pub meow: u32,
}

impl Serialization for NewClientPacket {
    fn serialize(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        let _ = out.write_u32::<BigEndian>(self.meow);

        return out;
    }

    fn deserialize(&mut self, data: &mut std::io::Cursor<Vec<u8>>) -> () {
        self.meow = data.read_u32::<BigEndian>().unwrap();
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
    
    fn operate(&self, game_state: &mut GameState, client_stream: Box<TcpStream>) -> Result<Box<dyn Packet>, bool> {
        let mut i = self.client_id;
        loop {
            if !game_state.clients.contains_key(&i) {
                break;
            }
            i += 1;
        }

        let new_client = Client { id: i, position: Pos::default(), stream: client_stream };
        game_state.clients.insert(i, new_client);
        println!("New client with ID {}, meow: {}", i, self.meow);

        let mut output_packet = NewClientPacket::default();
        output_packet.meow = self.meow;
        output_packet.client_id = i;

        return Ok(Box::new(output_packet));
    }

    fn get_new(&self) -> Box<dyn Packet> {
        return Box::new(NewClientPacket::default());
    }
}