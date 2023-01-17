use hw_008::entities::devices::{Device, Socket, Thermometer};
use hw_008::entities::house::{Home, Room};

/// Super dummy integration test. I'm tired to writing the texts here, so, I'll just leave it
/// `as-is` with the hope that sometime in the future we'll come up with the proper tests here.
/// So, please believe me, it will be fixed
#[test]
fn first_integration_test_ever() {
    let living_room = Room::build()
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
        .build();
    assert!(living_room.is_ok());

    let living_room = living_room.expect("Unable to create the living room");

    let kitchen = Room::build()
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
        .build();

    assert!(kitchen.is_ok());

    let kitchen = kitchen.expect("Unable to create the kitchen");

    let house = Home::build()
        .with_name("Home #1")
        .with_description("Moscow's home")
        .with_room(living_room)
        .with_room(kitchen)
        .build()
        .expect("Unable to create a smart house");

    let rooms = house.get_rooms();
    assert_eq!(rooms.len(), 2);

    let devices = house
        .get_devices(&rooms[0])
        .expect("Unable to get devices list");
    assert_eq!(devices.len(), 2)
}
