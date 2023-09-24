mod rect;
use rect::Rectangle;


trait Info {
    fn log(&mut self);
}


impl Info for Rectangle{
    fn log(&mut self){
        let area = self.area();
        println!("Length: {:?}\nBreadth: {:?}\nArea: {:?}\n", self.length, self.breadth, area);
    }
}

fn main() {
    println!("Rust Objects!");
    
    let mut polygon = Rectangle::new (30, 20);

    polygon.log();
    polygon.scale(2);
    polygon.log();
}