// #![allow(dead_code)]

use hw_004::entities::devices::{socket::Socket, thermometer::Thermometer};
use hw_004::entities::house::{home::Home, room::Room};
use hw_004::entities::reportable::Reportable;

fn main() {
    let house = Home::build()
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
    println!("{house}");

    println!("=========== Rooms ===============");
    let rooms = house.get_rooms();
    for room in rooms {
        println!("{room}");
    }
    println!("=========== Devices =============");
    let devices = house
        .get_devices(&rooms[0])
        .expect("Unable to get devices list");
    for device in devices {
        println!("{device}");
    }

    println!("=========== Report ===============");
    println!("{}", house.report());
}
