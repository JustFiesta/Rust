use std::io::stdin;

pub fn fill_array_with_usr_input(usr_input_array: &mut Vec<i32>){

    println!("Input some numbers to test sorting algorithm (type \"done\" to start sorting)");

    //loop and add numbers till user types exit
    loop {
        //container to usr input so we can parse it to i16
        let mut usr_number = String::new();

        //standard input ...
        stdin().read_line(&mut usr_number).expect("Failed to read input");
        
        //trim from whitespaces
        let usr_number = usr_number.trim();

        if usr_number == "done"  {
            break;
        }
        
        //parse String to i32, and unwrap it
        let parsed_number: i32 = usr_number.parse::<i32>().expect("Invalid input");

        //add number to array
        usr_input_array.push(parsed_number);
    }
}