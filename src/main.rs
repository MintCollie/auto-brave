//extern crate autopilot;
use std::process::Command;
use enigo::*;
use std::{thread, time};

fn main() {
    //declaring varables
    let mut enigo = Enigo::new();
    let ten_millis = time::Duration::from_millis(10);
    let _now = time::Instant::now();


    // spawns new instance of brave browser
    Command::new("brave")
            .spawn()    //spawns it
            .expect("brave failed to start"); // or tells it that it failed
    thread::sleep(ten_millis);
    enigo.key_click(Key::Layout('t'));
}