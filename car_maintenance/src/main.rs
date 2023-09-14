use std::io;

fn main() {
    loop{
        //Menu for app
        println!("Let's begin to monitor ur cars health!");
        println!("--------------------------------------");
        
        //Main menu options
        println!("Main Menu: (1-3)");
        println!("1. Log In");
        println!("2. Sign In");
        println!("3. Exit");
        
        match get_user_input_int() {
            Ok(choice) => match choice {
                1 => {
                    println!("logging in...");
                    // TODO: log_in();
                    // println!("Hello {}, nice to have you back!", logged_user.login());
                }
                2 => {
                    println!("We're glad to have you!");
                    // TODO: sign_in();
                    // println!("Hello {}, nice to have with us!", logged_user.login());

                }
                3 => {
                    print!("See ya soon!");
                    break; // just exit loop => program
                }
                _ => println!("Mismatched choice number! Try again"),
            }
            Err(err) => eprintln!("Error: {}", err), // go to standard error
        }
        
        //default panel after logging in
        
        println!("Choose option (1-9):");
        println!("1. Show next maintenance");
        println!("2. Show history");
        println!("3. Show odometer");
        println!("4. Add maintenance");
        println!("5. Edit existing maintenance");
        println!("6. Remove existing maintenance");
        println!("8. Vehicle options");
        println!("9. Account options");

        match get_user_input_int() {
            Ok(choice) => match choice {
                1 => todo!(),
                2 => todo!(),
                3 => todo!(),
                4 => todo!(),
                5 => todo!(),
                6 => todo!(),
                
                8 => todo!(),
                9 => todo!(),
                _ => println!("Mismatched choice number! Try again"),
            }
            Err(err) => eprintln!("Error: {}", err),
        }
        
        //vehicle panel
        println!("Choose option (1-4):");
        println!("1. Change default vehicle");
        println!("2. Add new vehicle");
        println!("3. Edit existing vehicle");
        println!("4. Remove vehicle");

        match get_user_input_int() {
            Ok(choice) => match choice {
                1 => todo!(),
                2 => todo!(),
                3 => todo!(),
                4 => todo!(),

                _ => println!("Mismatched choice number! Try again"),
            }
            Err(err) => eprintln!("Error: {}", err),
        }
        

        //account menage panel
        println!("Choose option (1-9):");
        //TODO
        // println!("1.  default vehicle");
        // println!("2. Add new vehicle");
        // println!("3. Edit existing vehicle");
        println!("9. Remove account");

        match get_user_input_int() {
            Ok(choice) => match choice {
                1 => todo!(),
                2 => todo!(),
                3 => todo!(),
                4 => todo!(),

                9 => todo!(),
                _ => println!("Mismatched choice number! Try again"),
            }
            Err(err) => eprintln!("Error: {}", err),
        }
        

    }
}

//Menage user input
fn get_user_input_int() -> Result<i32, io::Error> {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).unwrap();

    match input.trim().parse() {
        Ok(number) => Ok(number),
        Err(_) => Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid input! Please enter a valid number")),
    }
}
fn get_user_input_str() -> Result<String, io::Error> {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).unwrap();
    
    let trimmed_input = input.trim().to_string();

    if trimmed_input.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Your input is empty!"));
    }
    Ok(trimmed_input)
}

