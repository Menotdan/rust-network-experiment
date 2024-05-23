use crate::{network::{packet::Packet, serialization::Serialization}, types::pos::Pos};

#[derive(Default)]
pub struct MovePacket {
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
        return 0;
    }

    fn get_mod_id(&self) -> u32 {
        return 0;
    }
}