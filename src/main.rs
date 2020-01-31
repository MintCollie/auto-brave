extern crate autopilot;
use std::process::Command;

fn main() {


    // spawns new instance of brave browser
    Command::new("brave")
            .spawn()    //spawns it
            .expect("brave failed to start"); // or tells it that it failed

            
}