use crate::menage::Menage;
use crate::vehicles::vehicles::Vehicle;
struct User{
    user_id: u32,
    name: String,
    login: String,
    password: String,
    email: String,
    phone: u128,
    role: UserRole,
    default_vehicle: Vehicle,
}

enum UserRole{
    User,
    Admin,
}

impl Menage for User {
    fn add(&mut self) {
        todo!()
    }

    fn remove(&mut self) {
        todo!()
    }

    fn edit(&mut self) {
        todo!()
    }
}