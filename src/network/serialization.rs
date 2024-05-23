use std::io::Cursor;

pub trait Serialization {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(&mut self, data: &mut Cursor<Vec<u8>>) -> ();
}