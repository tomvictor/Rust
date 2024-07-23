use std::fmt::Display;

struct Location<T, U> {
    latitude: T,
    longitude: U,
}

impl<T: Display, U: Display> Location<T, U> {
    pub fn display(&self) {
        println!("latitude: {}\nlongitude: {}", self.latitude, self.longitude);
    }
}

fn main() {
    println!("Generics advanced!");

    let location = Location {
        latitude: 1.1 as f32,
        longitude: 2 as u32,
    };
    location.display();
}
