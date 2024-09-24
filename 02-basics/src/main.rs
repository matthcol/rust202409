use std::ops::Range;

fn play_with_string(){
    let city = "Toulouse";
    let city2: &str = "Tournefeuille";
    let city3 = "東京";
    println!("City = {0}", city);
    println!("Another city = {}", city2);
    println!("I ❤️ {city3}");
    println!("I \u{2665} {city3}");
    let french_sentence = "Vous avez mangé des œufs à L'Haÿ-les-Roses, ça fait du bien";
    println!("{french_sentence}");
    println!("Lengths (bytes): {}, {}, {}", city.len(), city3.len(), "œuf".len());
    println!("Lengths (graphemes): {}, {}, {}", 
            city.chars().count(), 
            city3.chars().count(), 
            "œuf".chars().count()
    );
    let mut slice = &city[..3];
    println!("Slice = {}", slice);
    slice = &city[3..6];
    println!("Slice = {}", slice);
    slice = &city[6..];
    println!("Slice = {}", slice);
    slice = &city[..];
    println!("Slice = {}", slice);
    slice = &city3[..3];
    println!("Slice = {}", slice);
    
    // wrong cut => panic
    // slice = &city3[..2];
    // println!("Slice = {}", slice);

    // c: Option<char>
    let mut c = city3.chars().nth(0);
    println!("One optional char: {:?}", c);
    c = city3.chars().nth(300); 
    println!("One optional char: {:?}", c);

    for g in french_sentence.chars() {
        println!("- one char = {g}")
    }
}

fn play_with_integer(){
    let price = 126;
    let price2: u64 = 10_000_000_000;
    let price3 = 15_000_000_000_u64;
    println!("prices: {}, {}, {}", price, price2, price3);
    // price = 128; // Error: price is constant
    let mut price4: u64;
    let mut price5 = 256;
    
    // Error: non assigned variable
    // println!("modifiable prices: {}, {}", price4, price5);
    println!("modifiable prices: {}", price5);
    price4 = 257;
    println!("modifiable prices: {}, {}", price4, price5);
    price4 += 1;
    price5 = 258;
    println!("modifiable prices: {}, {}", price4, price5);
    price4 *= 2;
    println!("modifiable price: {}", price4);

    // operators
    // https://doc.rust-lang.org/book/appendix-02-operators.html
    price4 = ((price5 * 2) + 3) / 4 - 5;
    println!("modifiable price: {}", price4);
    price4 = price4 % 10;
    println!("modifiable price: {}", price4);

    let mut _x: u32; // unused variable

    // comparison operators: < <= > >= == !=
    let ok = price4 < 10;
    println!("Ok: {}", ok);
    println!("Ok: {ok}");

    let x8: u8 = 25;
    //let _x64: u64 = x8; // no implicit conversion  
    let _x64: u64 = x8 as u64;

    let _r = 25_u64..42_u64;
    let _r2: Range<u8>= 25..42;

    for i in 25_u64..42_u64 {
        println!("* {i}")
    }

    println!("Price: {}", price4);
    if price4 < 10 {
        println!("All good 10")
    }

    if price4 < 100 {
        println!("All good 100")
    } else {
        println!("No good 100")
    }

    if price4 < 1000 {
        println!("All good 1000")
    } else if price4 < 10000 {
        println!("All good 10_000")
    } else {
        println!("No good")
    }

    let mut x = 10i8;
    while x >= 0 {
        println!("Soon: {x}");
        x -= 1;
    }
    println!("Boom !");
}

fn play_with_float(){
    // floating numbers according to 
    let mut price = 0.1_f32;
    println!("prices: {}, {}, {}", price, 2.0_f32 * price, 3.0_f32 * price);
    println!("prices: {:.8}, {:.8}, {:.8}", price, 2.0_f32 * price, 3.0_f32 * price);
    price *= 1E30_f32;
    println!("big price: {}", price);
    price *= 1E30_f32;
    println!("big price: {}", price);

    let freq = 1E15_f64;
    println!("Log frequence = {}", freq.log10());

    let freq_int = 10_000_u16;
    let range = (freq_int as f32).log10() as u16;
    println!("log10({freq_int}) = {range}")
}

fn play_with_pattern_matching() {
    let x = 8u8;

    println!("First match:");
    match x {
        _ => println!("Every 8bit integer is good: {x}")
    }
    println!();

    println!("Other matches:");
    for y in 0..=15 {
        print!(" - {y}: ");
        match y {
            0 => println!("Case 0"),
            3 => println!("Case 3"),
            1..=5 => println!("Case range 1 to 5 (except 3)"),
            // 3|6|8|11 => println!("Case list 3,6,8,11"), // 3 unreachable pattern
            6|8|11 => println!("Case list 6,8,11"),
            // 3 => println!(), // unreachable pattern
            _ if y % 3 == 0 => println!("Case multiple of 3 (except 0, 3, 6)"),
            _ => println!("Default case")
        }
    }
    println!();


}

/**
 * 
 */
fn main() {
    // Display a message in the console
    println!("Hello, world!");
    play_with_float();
    play_with_string();
    play_with_integer();
    play_with_pattern_matching();
}
