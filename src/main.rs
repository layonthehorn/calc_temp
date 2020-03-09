use std::io;

fn main() {

    let mut convert_type= String::new();
    println!("Chose to convert to cel or far, (c/f).");
    io::stdin().read_line(& mut convert_type)
        .expect("Failed to read line.");

    // using trim because there is a hidden new line if you don't
    let convert_type = convert_type.trim();


    // if they enter an accepted input continue
    if convert_type == "f" || convert_type == "c" {
        let number = loop {
            // creates a new string mutable variable
            let mut input = String::new();
            // tells you what is going on
            if convert_type == "c" {
                // if you picked cel
                println!("Enter a temp to convert to Cel. ");
            } else if convert_type == "f" {
                // if you picked far
                println!("Enter a temp to convert to Far. ");
            }

            // tries to get a number from the user
            io::stdin().read_line(&mut input)
                .expect("Failed to read line.");
            // if you did not supply a good number it tries again
            let input: f64 = match input.trim().parse() {
                // if its ok then it returns it
                Ok(num) => num,
                // otherwise it asks again
                Err(_) => continue,
            };
            // ends the loop and returns the number
            break input;
        };

        if convert_type == "c"{
            let final_temp = (number - 32.0) * 5.0/9.0;
            println!("{} cel in far is {}", number, final_temp)
        }else if convert_type == "f" {
            let final_temp = (number * 9.0/5.0) + 32.0;
            println!("{} far in cel is {}", number, final_temp)
        }
    }else{
            // if neither of the above match then we quit and return zero
            println!("Error, not acceptable input.");
        }










}
