#[derive(Debug)]
struct Location<T, U> {
    latitude: T,
    longitude: U,
}

fn main() {
    println!("Generics advanced!");

    let location = Location {
        latitude: 1.1 as f32,
        longitude: 2 as u32,
    };
    println!("location: {:?}", location);
}
