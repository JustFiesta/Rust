use std::{io, collections::HashMap};
struct Item {
    number: u32,
    price: u32,
    weight: u32,
    profability: u32,
}
impl Item {
    fn set_profability(&mut self) {
        self.profability = (self.price / self.weight) as u32
    }
}
fn main() {
    

    println!("------- Decyzyjny problem plecakowy ------");
    println!("Podaj nieprzekraczalną wagę plecaka: ");
    let max_weight = match input_string() {
        Ok(value) => value,
        Err(err) => panic!("{}", err),
    };

    println!("Dodaj przedmioty (jak skończysz wpisz q)");
    let mut items_array: Vec<Item> = vec![];

    loop {
        let mut i: u32 = 1;
        println!("Przedmiot nr {}", i);
        println!("Cena: ");
        let price: String = match input_string() {
                Ok(value) => value,
                Err(err) => break,
        };
        println!("Waga: ");
        let weight: String  = match input_string() {
            Ok(value) => value,
            Err(err) => break,
        };

        let mut item: Item = Item { number: i, price: price.parse::<u32>().unwrap(), weight: weight.parse::<u32>().unwrap(), profability: 0 };
        item.set_profability();

        i += 1;
        items_array.push(item);   
    }

    println!("Kalkuluje opłacalność...");
    let mut profability_hashmap: HashMap<u32, u32> = HashMap::new();
    for item in &items_array{
        profability_hashmap.insert(item.number, item.profability);
    }
    let mut sorted_items: Vec<_> = profability_hashmap.into_iter().collect();
    sorted_items.sort_by(|a, b| b.1.cmp(&a.1));

    

}
fn input_string() -> Result<String, io::Error> {
    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

    if input.is_empty() { 
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Empty input"))
    }

    let trimmed_input = input.trim().to_string();

    Ok(trimmed_input)
}