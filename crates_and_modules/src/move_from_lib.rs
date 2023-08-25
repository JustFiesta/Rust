pub mod front_of_house{
    mod hosting {
        pub fn add_to_waitlist() {}
        pub fn seat_at_table() -> String {
            String::from("sit down please")
        }   
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
        fn complain() {}
    }
}
pub mod back_of_house{
    fn fix_incorrect_order() {}
    pub fn cook_order() {}
    fn serve_order() {}
}

pub fn eat_at_restaurant() -> String {
    crate::front_of_house::hosting::add_to_waitlist();
    
    crate::back_of_house::cook_order();

    String::from("yummy yummy!")
}


