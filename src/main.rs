use std::io;

fn main() {
    println!("Input a temperature");

   let temperature: f32 = loop {

        //use a loop to ensure we get a number
        let mut temperature = String::new();
    
        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");
    
        //convert string to f32
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

    println!("unit is {unit}");
    let unit = unit.trim().to_lowercase();

    if unit == "c" {
        let temperature = celius_to_fahrenheit(temperature);
    } else {
        let temperature = fahrenheit_to_celcius(temperature);
    }

}

fn celius_to_fahrenheit(t: f32) -> f32 {
    t * 9.0/5.0 + 32.0
}

fn fahrenheit_to_celcius(t: f32) -> f32 {
    let t = t - 32.0;
    t * 5.0 / 9.0
}
