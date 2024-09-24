

fn display_array_slice(slice: &[i32]){
    println!("Slice: {}", slice.len());
    for x in slice {
        println!(" - {x}")
    }
    println!()
}

fn play_with_arrays(){
    let _temperatures: [i32;10]; // non initialized
    let _temperatures2: [i32; 10] = [0i32; 10];
    let _temperatures3: [i32; 10] = [0; 10];
    let mut temperatures = [0i32; 10];
    let temperatures4: [i32; 5] = [-2, 4, 5, 23, -12];
    println!("Temperatures: {temperatures:?}");
    println!("Temperatures (v4): {temperatures4:?}");
    temperatures[0] = 33;
    println!("Temperatures: {temperatures:?}");
    println!("Temperatures[0] = {}", temperatures[0]);

    // slices on arrays
    // array: [i32,5] => slice: &[i32]
    println!("Temperatures (v4) [..3] = {:?}", &temperatures4[..3]);

    display_array_slice(&temperatures[..4]);
    display_array_slice(&temperatures[4..]);
    display_array_slice(&temperatures4[..3]);
    display_array_slice(&temperatures4[3..=4]);

    let slice = &temperatures4[..4];
    // let slice2 = slice[..2];
    display_array_slice(slice);
    // display_array_slice(slice2);

    println!("Array:");
    for x in temperatures4 {
        println!("* {x}")
    }
    println!();

    println!("Length: {}", temperatures4.len())
}

fn generate_tuple()-> (i32, &'static str){
    (-12i32, "tuesday")
}

fn play_with_tuples(){
    let temperature_day = (24i32, "monday");
    println!("Temperature and day: {:?}", temperature_day);
    println!("Temperature {} and day: {}", temperature_day.0, temperature_day.1);
    let (temperature, day) = temperature_day;
    println!("Temperature {} and day: {}", temperature, day);

    let (temperature, day) = generate_tuple();
    println!("Temperature {} and day: {}", temperature, day);
}

fn play_with_enum_option(){
    let city = "Toulouse";
    let mut pos = 3;
    let mut opt_letter: Option<char> = city.chars().nth(pos);
    println!("Found: {:?}", opt_letter);
    println!("Found: {:?}", opt_letter.unwrap());

    pos = 128;
    opt_letter = city.chars().nth(pos);
    println!("Found: {:?}", opt_letter);
    // println!("Found: {:?}", letter.unwrap()); => panic
    println!("Found: {:?}", opt_letter.unwrap_or('A'));

    // shadowing of mutable variable pos => foreach variable
    for pos in [1usize, 4, 10, 25, 103]{
        println!("Position: {pos}");
        opt_letter = city.chars().nth(pos);

        // 1st method
        if opt_letter.is_none() { // alt. is_some()
            println!(" * Letter not found");
        } else {
            println!(" * Letter found: {}", opt_letter.unwrap());
        }

        // 2nd method
        if let Some(letter) = opt_letter  {
            println!(" ~ Letter found (2): {}", letter);
        } else {
            println!("~ Letter not found (2)");
        }
    }
    // end of shadowing

    // back to mutable variable pos
    pos = 4;
    println!("Unshadow pos: {pos}")
}



fn _play_with_struct(){

}

fn main() {
    play_with_arrays();
    play_with_tuples();
    play_with_enum_option()
}
