use std::io;
use std::io::Write;
use std::io::stdin;

fn main() {
    println!("Welcome to the temperature converter!");
    loop {
        let mut temp = String::new();
        print!("Please enter the temperature amount you want to convert: ");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut temp) {
            Ok(_) => {
                let temp: f32 = match temp.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please enter the temperature.");
                        continue;
                    }
                };
                loop {
                    let mut type_from = String::new();
                    loop {
                        print!("Please say to what type you want to convert from. celsius, fahrenheit or kelvin: ");
                        io::stdout().flush().unwrap();
                        stdin().read_line(&mut type_from).unwrap();
                        type_from = type_from.to_lowercase();
                        if let Some('\n')=type_from.chars().next_back() { type_from.pop(); }
                        if let Some('\r')=type_from.chars().next_back() { type_from.pop(); }
                        if type_from != "celsius" && type_from != "fahrenheit" && type_from != "kelvin" {
                            println!("Please type either \"celsius\", \"fahrenheit\" or \"kelvin\".");
                        } else { break; }
                    }
                    

                    let mut type_to = String::new();
                    loop {
                        print!("Please say to what type you want to convert to. celsius, fahrenheit or kelvin: ");
                        io::stdout().flush().unwrap();
                        stdin().read_line(&mut type_to).unwrap();
                        type_to = type_to.to_lowercase();
                        if let Some('\n')=type_to.chars().next_back() { type_to.pop(); }
                        if let Some('\r')=type_to.chars().next_back() { type_to.pop(); }
                        if type_to != "celsius" && type_to != "fahrenheit" && type_to != "kelvin" {
                            println!("Please type either \"celsius\", \"fahrenheit\" or \"kelvin\"");
                        } else { break; }
                    }

                    if type_from == "celsius" {
                        if type_to == "fahrenheit" {
                            println!("The amount you specified is {} degrees converted from celsius to fahrenheit.", temp*1.8+32.0); 
                        } else if type_to == "kelvin" {
                            println!("The amount you specified is {} degrees converted from celsius to kelvin.", temp+273.15);
                        } else {
                            println!("Why do you want to convert celsius to celsius you dummy!");
                        }
                        break;
                    } else if type_from == "fahrenheit" {
                        if type_to == "celsius" {
                            println!("The amount you specified is {} degrees converted from fahrenheit to celsius.", 0.5556*(temp-32.0)); 
                        } else if type_to == "kelvin" {
                            println!("The amount you specified is {} degrees converted from fahrenheit to kelvin.", 0.5556*(temp+459.67));
                        } else {
                            println!("Why do you want to convert fahrenheit to fahrenheit you dummy!");
                        }
                        break;
                    }
                }
                break;
                
            }
            Err(_) => {
                println!("Please enter the temperature.");
                continue;
            }
        }
    }
}
