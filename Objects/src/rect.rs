use std::clone::Clone;




#[derive(Clone)]
pub struct Rectangle {
    pub length:i32,
    pub breadth: i32,
}

impl  Rectangle {
    pub fn new(length:i32,breadth:i32) -> Self{
        Self{
            length,
            breadth
        }
    }
    
    pub fn area(&mut self) ->i32 {
        let area = self.length * self.breadth;
        area
    }

    pub fn scale(&mut self, factor:i32){
        self.length = self.length*factor;
        self.breadth = self.breadth*factor;
    }

    // pub fn log(&mut self){
    //     let area = self.area();
    //     println!("Length: {:?}\nBreadth: {:?}\nArea: {:?}\n", self.length, self.breadth, area);
    // }
}