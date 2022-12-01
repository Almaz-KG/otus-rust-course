use hw_004::entities::house::home::Home;
use hw_004::entities::house::room::Room;
use hw_004::entities::devices::thermometer::Thermometer;
use hw_004::entities::devices::socket::Socket;


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
    let socket = Socket::new(
        "Main light socket",
        "Located near the entry door");

    println!("{}", socket);
}

fn show_usage_of_thermometer_entity() {
    let thermometer = Thermometer::new(
        "A thermometer behind the window",
        "Super old mercury thermometer",
    );

    println!("{}", thermometer);
}


fn main() {
    show_usage_of_home_entity();
    show_usage_of_room_entity();
    show_usage_of_socket_entity();
    show_usage_of_thermometer_entity();
}