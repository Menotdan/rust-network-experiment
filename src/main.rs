mod network;
mod types;
mod core;

use std::io::Cursor;
use network::serialization::Serialization;
use network::packet;
use types::pos::Pos;

fn main() {
    let test = Pos { x: 10, y: 15 };
    let data = test.serialize();
    
    let mut test_out = Pos::default();
    test_out.deserialize(&mut Cursor::new(data));

    println!("X: {}, Y: {}", test_out.x, test_out.y);
}
