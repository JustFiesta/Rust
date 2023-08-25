use crates_and_modules::eat_at_restaurant;
pub mod front_of_house; // need to tell binary what im using from library

fn main() {
    assert_eq!(crates_and_modules::front_of_house::hosting::seat_at_table(), "sit down please");
    assert_eq!(eat_at_restaurant(),"yummy yummy!");
}