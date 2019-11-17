#![allow(dead_code)]
#![allow(non_snake_case)]

mod gba;
use gba::GBA;

fn main() {
    let mut gba = GBA::new();
    gba.run();
}
