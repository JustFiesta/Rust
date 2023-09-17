use crate::menage::Menage;

// Structure to represent vehicle
pub struct Vehicle{
    vehicle_id: u32,
    default: IsDefault,
    make: String,
    model: String,
    year: u16,
    mileage: u128,
    engine_displacement: f32,
    fuel_type: FuelType,
}

enum FuelType {
    Gasoline,
    Diesel,
    LPG,
    Hybrid,
    Electric,
}
enum IsDefault {
    Yes,
    No,
}


impl Menage for Vehicle {
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