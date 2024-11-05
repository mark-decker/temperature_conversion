//Mark Decker 5/11/2024
//Learning Rust
//rust-lang chapter 3

use std::io;

fn main() {
    println!("Input a temperature");

   let temperature: f32 = loop {  //use a loop to ensure we get a number within limits
                                  //just started rust unsure if this is good method

       let mut temperature = String::new();
    
       io::stdin()
           .read_line(&mut temperature)
           .expect("Failed to read line");
    
       //convert string to f32
       //this should not cause runtime error 
       //need different solution
       let temperature: f32 = temperature
           .trim()
           .parse()
           .expect("Please Enter a Number");

       if temperature > -273.0 {
           break temperature
       } else if temperature < 1000.0 {
           break temperature
       } else {
           println!("Temperature must be between -273 and 1000");
       }

    };
    //add a loop to read line until user inputs c or f

    //let mut unit = String::new();

    let unit = loop { 
        println!("Input the temperature unit.  Use C for celcius and F for fahrenheit");

        let mut unit = String::new();

        io::stdin()
            .read_line(&mut unit)
            .expect("Failed to read line");
   
        unit = unit.trim().to_lowercase();  //ensure the user input is lower case

        if unit == "c" {
            break unit
        } else if unit == "f" {
            break unit
        } else {
            println!("unit is not c or f");
        }

    };

    let converted_temperature: (f32, char) = {  
        if unit == "c" { 
            celius_to_fahrenheit(temperature)
        } else {
            fahrenheit_to_celcius(temperature)
        }
    };

    println!("The input temperature of {temperature} in {unit} is {} in {}",converted_temperature.0,converted_temperature.1);

}

fn celius_to_fahrenheit(t: f32) -> (f32, char) {
    (t * 9.0/5.0 + 32.0, 'f')
}

fn fahrenheit_to_celcius(t: f32) -> (f32, char) {
    let t = t - 32.0;
    (t * 5.0 / 9.0, 'c')
}
