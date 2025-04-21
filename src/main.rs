use std::io;

fn main() {
    let mut temperature_type = String::new();

    println!("What type of temperature are you converting from?\n");

    io::stdin()
        .read_line(&mut temperature_type)
        .expect("Failed to read input.");

    while (&mut temperature_type !in ["celcius", "farenheit"]) {
        println!("Temperature type must be of celcius or farenheit...\n");
    }
    
}
