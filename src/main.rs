use std::io;

mod helpers;

fn main() {
    let mut temperature_for_conversion: String = String::new();
    let mut temperature_for_conversion_float: f32 = 0.0;
    let mut temperature_scale: String = String::new();
    let mut is_valid_temperature: bool = false;

    // Get a valid temperature.
    while !is_valid_temperature {
        println!("Please provide the temperature for conversion:");

        io::stdin()
            .read_line(&mut temperature_for_conversion)
            .expect("Failed to read line.");

        temperature_for_conversion_float = match temperature_for_conversion
            .trim()
            .parse() {
            Ok(num) => {
                is_valid_temperature = true;
                num
            }
            Err(_) => {
                println!("You must provide a valid temperature.");
                temperature_for_conversion = String::new();
                continue;
            }
        };
    }


    // Get a valid temperature_scale.
    loop {
        println!("Please, provide the target temperature scale (F/C):");

        io::stdin()
            .read_line(&mut temperature_scale)
            .expect("Failed to read line.");

        temperature_scale = temperature_scale.trim().to_string();

        if helpers::is_valid_temperature_scale(&temperature_scale) {
            break;
        } else {
            temperature_scale = String::new();
            println!("You must provide 'F' for Fahrenheit or 'C' for Celsius, as the temperature scale.");
        }
    }


    // Make tha calculation based on the temperature_scale.
    if temperature_scale == "F" {
        println!("{} Celcius = {} Fahrenheit",
                 temperature_for_conversion_float,
                 helpers::convert(&temperature_for_conversion_float, &temperature_scale));
    }
    if temperature_scale == "C" {
        println!("{} Fahrenheit = {} Celcius",
                 temperature_for_conversion_float,
                 helpers::convert(&temperature_for_conversion_float, &temperature_scale));
    }
}
