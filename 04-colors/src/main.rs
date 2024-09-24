#[derive(Debug)]
enum Color {
    Green(u16),
    Orange,
    Red(f32, f32)
}

fn play_with_enum_color(){
    let mut color = Color::Orange;
    println!("Color: {color:?}");
    color = Color::Green(3);
    println!("Color: {color:?}");
    color = Color::Red(12.2, 1E14);
    println!("Color: {color:?}");

    // if let (1)
    if let Color::Green(cpt) = color {
        println!("Green during {cpt} hours !")
    }

    // if let (2)
    if let Color::Red(temperature, pression) = color {
        println!("Red at {temperature} °C and  {pression} Pa")
    }

    // play with match
    match color {

    }
}

fn main() {
    play_with_enum_color()
}
