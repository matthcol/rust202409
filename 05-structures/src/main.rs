use std::collections::BTreeSet;

use compute::{in_range, inverted_square};
use ordered_float::OrderedFloat;

mod compute;

fn play_with_strings(){
    println!("** play_with_strings **");
    let city = String::from("Toulouse");
    println!("City: {city}");
    println!("City: {city:?}");
    println!("City: {city:#?}"); // pretty print debug

    let city2 = String::new(); 
    println!("{city2:?}");

    let mut city2 = String::new();
    city2.push('東');
    // NB: every function accepting &str accepts &String
    city2.push_str("京都");
    println!("{city2:?}");
    city2.push_str(" and ");
    city2.push_str(&city);
    println!("{city2:?}");

    let decoupe: Vec<&str> = city2.split(" and ").collect();
    println!("{:?}", decoupe);

    let decoupe: String = city2.split(" and ").collect();
    println!("{:?}", decoupe);

    let decoupe = city2.split(" and ").collect::<String>();
    println!("{:?}", decoupe);

    println!("1st char: {:?}", city2.chars().nth(0));

    let new_city = city2.replace(" and ", ", ");
    println!("{new_city}");

    let l = "東京都".len();
    city2.replace_range(..l, "Pau");
    println!("{city2:?}");

    city2 = String::from("Tournefeuille");
    println!("{city2:?}");
    println!()
}

fn play_with_collections(){
    // https://doc.rust-lang.org/std/collections/index.html
    println!("** play_with_collections **");
    let mut speeds: Vec<f64> = vec![5.3, 3E8, -50.5];
    println!("Speeds: {speeds:?}");
    println!("speeds: length={}, capacity={}", speeds.len(), speeds.capacity());
    for i in 1..10 {
        speeds.push(i as f64 * 5.5 -3.25);
        println!("speeds: length={}, capacity={}", speeds.len(), speeds.capacity());
    }
    println!("Speeds: {speeds:?}");

    let mut other_speeds: Vec<f64> = vec![0.0;1_000_000];
    println!("other_speeds: length={}, capacity={}", other_speeds.len(), other_speeds.capacity());
    // fill vec
    for i in 1..other_speeds.len(){
        other_speeds[i] = i as f64 * 5.5 -3.25;
    }
    other_speeds.push(1.0); // capacity * 2 !!!
    println!("other_speeds: length={}, capacity={}", other_speeds.len(), other_speeds.capacity());

    // empty vec of capacity 1_000_000 (length=0)
    let another_speeds: Vec<f64> = Vec::with_capacity(1_000_000);
    println!("another_speeds: length={}, capacity={}", another_speeds.len(), another_speeds.capacity());

    let no_speed: Vec<f64> = Vec::new();
    println!("no_speed: length={}, capacity={}", no_speed.len(), no_speed.capacity());
}



fn play_with_map_reduce() {
    let speeds: Vec<f64> = vec![32.5, 5.3, 3E8, -50.5, 25.6];

    // (with mut variable) speeds.pop_if(|speed| *speed < 0.0);

    // anonymous functions (λ function)
    // Rust:            |x| x*2
    // Python:          lambda x: x*2
    // Javascript/C#:   x => x*2
    // Java:            x -> x*2
    // C++:             [](auto x){ return x*2;}
    let new_speeds: Vec<f64> = speeds.iter()
        .filter(|speed| **speed > 0.0)
        .map(|speed| speed * 2.0)
        .collect();
    println!("Speeds filtered and transformed: {new_speeds:?}");

    let rounded_speeds: Vec<u64> = speeds.iter()
        .filter(|speed| **speed > 0.0)
        .map(|speed| speed.floor() as u64)
        .collect();
    println!("Rounded positive speeds: {rounded_speeds:?}");

    let result: f64 = speeds.iter()
        .filter(|speed| **speed > 0.0)
        .map(|speed|{
            let x = speed.sqrt();
            x.ceil()
        })
        .sum();
    println!("Result:: {result}");

    // map/reduce pipeline with named functions
    let data: Vec<f64> = speeds.iter()
        .filter(|speed| in_range(**speed, 0.0, 300.0))
        .map(inverted_square)
        .collect();

    println!("Data: {data:?}");
    
    // sort data
    let mut temperatures: Vec<i32> = vec![5, 12, 3, 125, -2, 20, 18, 54, 12, 90];
    temperatures[..4].sort();
    println!("Sorted temperatures: {temperatures:?}");

    temperatures.sort(); // implicitly on slice ..
    println!("Sorted temperatures: {temperatures:?}");

    let mut sorted_speeds: Vec<f64> = speeds.clone(); // cf: cf traits Copy, Clone
    sorted_speeds.sort_by_key(|speed| OrderedFloat(*speed));

    println!("Sorted speeds: {sorted_speeds:?}");

    let mut sorted_speeds: BTreeSet<i64> = speeds.iter()
        .map(|speed| speed.floor() as i64)
        .collect();
    println!("Sorted speeds (btree): {sorted_speeds:?}");
    sorted_speeds.insert(0);
    println!("Sorted speeds (btree): {sorted_speeds:?}");

    let v = sorted_speeds.iter()
        .nth(4)
        .unwrap();

    let values: Vec<i64> = sorted_speeds.iter()
        .skip(3)
        .take(2)
        .copied()
        .collect();

    // wrong slice: range end index 30 out of range for slice of length 10
    let slice =  &temperatures[..30];
    println!("{slice:?}");

    // wrong slice with get => None
    let slice =  temperatures.get(..30); // => None
    println!("{slice:?}");
}

fn main() {
    play_with_strings();
    play_with_collections();
    play_with_map_reduce()
}
