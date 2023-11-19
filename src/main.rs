use std::io;

fn main() {
    println!("Welcome to the Temperature Converter.");
    println!("Please use the following options to convert your temperature.");
    println!("1. Fahrenheit to Celsius");
    println!("2. Celsius to Fahrenheit");

    let mut converted_temp: f64 = 0.0;
    let mut unit: &str;

    loop {
        let mut selection = String::new();

        io::stdin()
            .read_line(&mut selection)
            .expect("Failed to read selection");

        let selection: u8 = match selection.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if selection == 1 {
            println!("You have chosen to convert Fahrenheit to Celsius.");
            unit = "Celcius";
        } else if selection == 2 {
            println!("You have chosen to convert Celcius to Fahrenheit");
            unit = "Fahrenheit";
        } else {
            println!("That is not a valid option. Please use one of the following options.");
            println!("1. Fahrenheit to Celsius");
            println!("2. Celsius to Fahrenheit");
            continue;
        }

        println!("Please enter the temperature value.");

        let mut temperature = String::new();

        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read the temperature");

        let temperature: f64 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if selection == 1 {
            converted_temp = convert_fahrenheit_to_celcius(temperature);
        } else if selection == 2 {
            converted_temp = convert_celcius_to_fahrenheit(temperature);
        }
        break;
    }

    println!("The converted temperature is {converted_temp} degrees {unit}");
}

fn convert_celcius_to_fahrenheit(celcius: f64) -> f64 {
    (9.0 * celcius / 5.0) + 32.0
}

fn convert_fahrenheit_to_celcius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 9.0 / 5.0
}
