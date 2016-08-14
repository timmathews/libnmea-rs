extern crate libnmea;

use libnmea::*;

fn main() {
    let pgns = pgn_list();

    println!("{:?}", pgns)
}
