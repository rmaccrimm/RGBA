#![allow(dead_code)]
#![allow(non_snake_case)]

mod gba;
use gba::GBA;

struct Tmp {
    x: u32
}

fn main() {
    let mut gba = GBA::new();
    gba.run();
}
