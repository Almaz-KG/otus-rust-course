#![allow(dead_code)]

use crate::device::{Reportable, Socket, Thermometer};
use crate::house::{House, Room};

mod device;
mod house;

fn main() {
    let house1 = House::build()
        .with_name("Home #1")
        .with_description("Moscow's home")
        .with_room(
            Room::build()
                .with_name("Living room")
                .with_description("Living room with 48 sq meter size")
                .with_device(Box::new(Socket::new(
                    "Main light socket",
                    "Located near the entry door",
                )))
                .with_device(Box::new(Socket::new(
                    "Second light socket",
                    "Located near the window",
                )))
                .build()
                .expect("Unable to create the living room"),
        )
        .with_room(
            Room::build()
                .with_name("Kitchen")
                .with_description("The kingdom of my wife")
                .with_device(Box::new(Socket::new(
                    "The light socket",
                    "Located at the entry door",
                )))
                .with_device(Box::new(Thermometer::new(
                    "A thermometer behind the window",
                    "Super old mercury thermometer",
                )))
                .build()
                .expect("Unable to create the kitchen"),
        )
        .build()
        .expect("Unable to create a smart house");

    println!("=========== Display ===============");
    println!("=========== Home ===============");
    println!("{house1}");

    println!("=========== Rooms ===============");
    let rooms = house1.get_rooms();
    for room in rooms {
        println!("{room}");
    }
    println!("=========== Devices =============");
    let devices = house1
        .get_devices(&rooms[0])
        .expect("Unable to get devices list");
    for device in devices {
        println!("{device}");
    }

    println!("=========== Report ===============");
    println!("{}", house1.report());
}
