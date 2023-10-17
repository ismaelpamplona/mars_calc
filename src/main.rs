use std::io;

fn main() -> io::Result<()> {
    println!("\nEnter your weight (kg): ");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let weight: f32 = input.trim().parse().unwrap();
    let weight_on_mars = calculate_weight_on_mars(weight);
    println!("\n >>> Weight on mars: {} <<<\n", weight_on_mars);
    Ok(())
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    weight / 9.81 * 3.711
}
