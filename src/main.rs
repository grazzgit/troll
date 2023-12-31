fn main() {
    let mut mars_weight = calculate_our_weight_on_mars(75.0);
    mars_weight = mars_weight * 1000.0;
    println!("Our weight on Mars is {} grams.", mars_weight);
}

fn calculate_our_weight_on_mars(weight: f32) -> f32 {
    // calculate our weight on mars
    weight / 9.81 * 3.711
}
