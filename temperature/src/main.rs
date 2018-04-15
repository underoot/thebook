fn main() {
    println!("Value is {}", to_fahrenheit(-273.15));
}

const CONVERT_FACTOR: f64 = (212.0 - 32.0) / 100.0;
const ICE_POINT: f64 = 32.0;

fn to_fahrenheit(deg: f64) -> f64 {
    deg * CONVERT_FACTOR + ICE_POINT
}

fn to_celsius(deg: f64) -> f64 {
    (deg - ICE_POINT) / CONVERT_FACTOR
}
