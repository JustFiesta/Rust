pub mod front_of_house;
pub mod back_of_house;

pub fn eat_at_restaurant() -> String {
    crate::front_of_house::hosting::add_to_waitlist();
    
    crate::back_of_house::cook_order();

    String::from("yummy yummy!")
}


