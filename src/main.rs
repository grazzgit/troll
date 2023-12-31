use std::io;

fn main() {
    println!("Enter your weight on Earth: ");
    let earth_weight = get_earth_weight_input();
    let mars_weight = calculate_our_weight_on_mars(earth_weight);
    println!("Your weight on Mars is {} grams.", mars_weight);
}

fn calculate_our_weight_on_mars(weight: f32) -> f32 {
    weight / 9.81 * 3.711
}

fn get_earth_weight_input() -> f32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}
