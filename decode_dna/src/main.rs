use std::io;

fn main() {
    println!("Give me an DNA sequence (ex. ATTGC, GTAT, etc.)");
    let mut user_dna = String::new();
    io::stdin().read_line(&mut user_dna).expect("Failed to read input");

    user_dna = user_dna.trim().to_uppercase();

    let mut output_dna = String::new();
    for char in user_dna.chars() {
        let complement = match char{
            'A' => 'T',
            'T' => 'A',
            'G' => 'C',
            'C' => 'G',
            _ => continue, 
        };
        output_dna.push(complement);
    }

    println!("Zmieniony łańcuch DNA: {}", output_dna);
}
