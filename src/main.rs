use std::io;

fn main() {
    println!("Input a temperature");

    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    //convert string to f32
    let temperature: f32 = temperature
        .trim()
        .parse() 
        .expect("Please input a number");

    println!("Input the temperature unit.  Use C for celcius and F for fahrenheit");

    let mut unit = String::new();

    io::stdin()
        .read_line(&mut unit)
        .expect("Failed to read line");

    unit = unit.to_lowercase();  //ensure the user input is lower case
    
}

fn celius_to_fahrenheit(t: f32) -> f32 {
    t * 9.0/5.0 + 32.0
}

fn fahrenheit_to_celcius(t: f32) -> f32 {
    let t = t - 32.0;
    t * 5.0 / 9.0
}
