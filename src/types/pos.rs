use crate::network::serialization::Serialization;

use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

#[derive(Default)]
pub struct Pos {
    pub x: u32,
    pub y: u32
}

impl Serialization for Pos {
    fn serialize(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        out.write_u32::<BigEndian>(self.x).unwrap();
        out.write_u32::<BigEndian>(self.y).unwrap();
        
        return out;
    }
    
    fn deserialize(&mut self, data: &mut Cursor<Vec<u8>>) -> () {
        self.x = data.read_u32::<BigEndian>().unwrap();
        self.y = data.read_u32::<BigEndian>().unwrap();
    }
}