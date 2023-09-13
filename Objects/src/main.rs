mod rect;

fn main() {
    println!("Rust Objects!");
    
    let mut polygon = rect::React {
        length: 30,
        breadth:20,
    };

    polygon.log();
    polygon.scale(2);
    polygon.log();

    // let rectangle_list:Vec<rect::React> = [].to_vec();
}