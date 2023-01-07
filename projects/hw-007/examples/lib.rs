#![allow(unused)]

use hw_007::entities::devices::{Device, Socket, Thermometer};
use hw_007::entities::house::{Home, Room};
use hw_007::entities::Reportable;

fn show_usage_of_home_entity() {
    let home = Home::build()
        .with_name("Moscow, Red Square #1")
        .with_description("My sweetie home")
        .build()
        .expect("Unable create home");

    println!("{}", home);
}

fn show_usage_of_room_entity() {
    let room = Room::build()
        .with_name("Living Room #1")
        .with_description("Here I sleep")
        .build()
        .expect("Unable create room");

    println!("{}", room);
}

fn show_usage_of_socket_entity() {
    let socket = Socket::new_with_description("Main light socket", "Located near the entry door");

    println!("{}", socket);
}

fn show_usage_of_thermometer_entity() {
    let thermometer = Thermometer::new_with_description(
        "A thermometer behind the window",
        "Super old mercury thermometer",
    );

    println!("{}", thermometer);
}

fn main() {
    let house = Home::build()
        .with_name("Home #1")
        .with_description("Moscow's home")
        .with_room(
            Room::build()
                .with_name("Living room")
                .with_description("Living room with 48 sq meter size")
                .with_device(Device::Socket(Socket::new_with_description(
                    "Main light socket",
                    "Located near the entry door",
                )))
                .with_device(Device::Socket(Socket::new_with_description(
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
                .with_device(Device::Socket(Socket::new_with_description(
                    "The light socket",
                    "Located at the entry door",
                )))
                .with_device(Device::Thermometer(Thermometer::new_with_description(
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
    println!("{}", house);

    println!("=========== Rooms ===============");
    let rooms = house.get_rooms();
    for room in rooms {
        println!("{}", room);
    }
    println!("=========== Devices =============");
    let devices = house
        .get_devices(&rooms[0])
        .expect("Unable to get devices list");
    for device in devices {
        println!("{}", device);
    }

    println!("=========== Report ===============");
    match house.report() {
        Ok(report) => println!("{}", report),
        Err(msg) => println!("Unable create a report due error: {}", msg),
    }
}
