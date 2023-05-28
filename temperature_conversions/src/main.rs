fn celcius_to_farenheight(temperature_in_celcius: i32) {
    let temperature_in_farenheight: i32 = (temperature_in_celcius * 9 / 5) + 32;
    println!(
        "The temperature in farenheight is: {}",
        temperature_in_farenheight
    );
}

fn farenheight_to_celcius(temperature_in_farenheight: i32) {
    let temperature_in_celcius: f64 = (temperature_in_farenheight - 32) as f64 * (5.0 / 9.0);
    println!("The temperature in celcius is: {}", temperature_in_celcius);
}

fn main() {
    celcius_to_farenheight(32);
    farenheight_to_celcius(212);
}
