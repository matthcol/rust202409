// module

use std::fmt::Display;

pub enum Dummy{
    Dumb,
    Dumber
}

#[derive(Debug)]
pub enum Color {
    Green(u16),
    Orange,
    Red(f32, f32)
}

impl Display for Color {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Green(duration) => f.write_fmt(
                format_args!("green ({duration})")
            ),
            Color::Orange => f.write_str("orange"),
            Color::Red(temperature, pression) => f.write_fmt(
                format_args!("red ({temperature}°C, {pression} Pa)")
            )
        }
   }
}

impl Color {
    pub fn to_numeric(&self) -> f32 {
        match self {
            Color::Green(duration) => *duration as f32,
            Color::Orange => f32::NAN,
            Color::Red(temperature, pression) => temperature * pression
        }

    }
}

pub fn string_to_color(color_str: &str) -> Option<Color> {
    match color_str {
        "green" => Some(Color::Green(0)),
        "orange" => Some(Color::Orange),
        "red" => Some(Color::Red(0.0, 0.0)),
        _ => None
    }
}