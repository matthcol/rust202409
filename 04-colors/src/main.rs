use color::{string_to_color, Color, Dummy};
// use color::*;

mod color;

fn _play_with_dummy(){
    let _dum = Dummy::Dumb;
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
        _ => println!("dummy case")
    }

    println!("Color with trait Display: {color}");

    let opt_color = string_to_color("green");
    if let Some(color) = opt_color {
        println!("Color read: {color}");
        let nb = color.to_numeric();
        println!("Nb from color: {nb}")
    }

    
}

fn main() {
    play_with_enum_color()
}
