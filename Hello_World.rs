fn main() {
    println!("Hello world!");
    let s = String::from("Hello world!");
    println!("{}", s);
    let moveS = s + "How are you?"; //Moved ownership of s to moveS - we cant use s after this line

    // println!("{}", s); - this print macro wont work
    println!("{}", moveS);
}