use clap::{Parser, ValueEnum};

/// Simple program to convert temperatures between Celcius and Farenheit
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Type of temperature to be converted from.
    #[arg(short = 'f', long = "from_temperature_type", value_enum)]
    from_type: TemperatureType,

    /// Type of temperature to be converted to.
    #[arg(short = 't', long = "to_temperature_type", value_enum)]
    to_type: TemperatureType,

    #[arg(short = 'v', long = "value")]
    value: f64,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
enum TemperatureType {
    Kelvin,
    Farenheit,
    Celcius,
}

fn calculate_from_kelvin(from_value: f64, to_type: TemperatureType) -> f64 {
    match to_type {
        TemperatureType::Kelvin => from_value,
        TemperatureType::Celcius => from_value - 273.15,
        TemperatureType::Farenheit => (from_value - 273.15) * (9.00 / 5.00) + 32.00,
    }
}

fn calculate_from_celcius(from_value: f64, to_type: TemperatureType) -> f64 {
    match to_type {
        TemperatureType::Kelvin => from_value + 273.15,
        TemperatureType::Celcius => from_value,
        TemperatureType::Farenheit => from_value * (9.00 / 5.00) + 32.00,
    }
}

fn calculate_from_farenheit(from_value: f64, to_type: TemperatureType) -> f64 {
    match to_type {
        TemperatureType::Kelvin => (from_value - 32.00) * (5.00 / 9.00) + 273.15,
        TemperatureType::Celcius => (from_value - 32.00) * (5.00 / 9.00),
        TemperatureType::Farenheit => from_value,
    }
}
fn main() {
    let args = Args::parse();
    let converted_temperature: f64;
    match args.from_type {
        TemperatureType::Kelvin => {
            converted_temperature = calculate_from_kelvin(args.value, args.to_type)
        }
        TemperatureType::Farenheit => {
            converted_temperature = calculate_from_farenheit(args.value, args.to_type)
        }
        TemperatureType::Celcius => {
            converted_temperature = calculate_from_celcius(args.value, args.to_type)
        }
    }
    println!(
        "The result of calculating from {:?} to {:?} is from {} to {}!",
        args.from_type, args.to_type, args.value, converted_temperature
    )
}
