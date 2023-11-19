use std::io;

fn main() {
    println!("Welcome to the Temperature Converter.");
    println!("Please use the following options to convert your temperature.");
    println!("1. Fahrenheit to Celsius");
    println!("2. Celsius to Fahrenheit");

    let mut converted_temp = 0.0;

    loop {
        let selection: u8 = match get_user_input("Select an option:").trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid selection. Please enter 1 or 2.");
                continue;
            }
        };

        if selection == 1 {
            println!("You have chosen to convert Fahrenheit to Celsius.");
        } else if selection == 2 {
            println!("You have chosen to convert Celsius to Fahrenheit");
        } else {
            println!("Invalid option. Please enter 1 or 2.");
            continue;
        }

        let temperature: f64 = match get_user_input("Please enter the temperature value:")
            .trim()
            .parse()
        {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid temperature. Please enter a valid number.");
                continue;
            }
        };

        if selection == 1 {
            converted_temp = convert_fahrenheit_to_celsius(temperature);
        } else if selection == 2 {
            converted_temp = convert_celsius_to_fahrenheit(temperature);
        }

        let unit = if selection == 1 {
            "Celsius"
        } else {
            "Fahrenheit"
        };

        println!(
            "The converted temperature is {:.2} degrees {}",
            converted_temp, unit
        );

        break;
    }
}

fn convert_celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn convert_fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
}
