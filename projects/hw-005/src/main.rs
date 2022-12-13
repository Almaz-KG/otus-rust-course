use hw_005::entities::devices::{Device, Socket, Thermometer};
use hw_005::entities::house::{Home, Room};
use hw_005::entities::Reportable;

fn main() {
    let house = Home::build()
        .with_name("Home #1")
        .with_description("Moscow's home")
        .with_room(
            Room::build()
                .with_name("Living room")
                .with_description("Living room with 48 sq meter size")
                .with_device(Device::Socket(Socket::new(
                    "Main light socket",
                    "Located near the entry door",
                )))
                .with_device(Device::Socket(Socket::new(
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
                .with_device(Device::Socket(Socket::new(
                    "The light socket",
                    "Located at the entry door",
                )))
                .with_device(Device::Thermometer(Thermometer::new(
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
