use plane::Plane;

mod plane;

fn play_with_planes() {
    // let plane: Plane = Plane::new();

    let type_ac = String::from("A321");
    let flight_level = 35;
    let max_speed = 0.77;
    let plane1 = Plane{type_ac, flight_level, max_speed};
    println!("Plane: {plane1:?}");

    let plane2: Plane = Plane{
        type_ac: String::from("A321"), 
        flight_level: 35, 
        max_speed: 0.77
    };
    println!("Plane: {plane2:?}");
    println!("Plane: {plane2:#?}");

    
}

fn main() {
    play_with_planes()
}
