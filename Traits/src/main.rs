struct Cat {
    name: String,
}

struct Dog {
    name: String,
}

struct Duck {
    name: String,
}

struct Horse {
    name: String,
}

trait Animal {
    // Definition
    // fn talk(&self) -> String;
    // Default Implementation
    fn talk(&self) -> String {
        // let voice = format!("Hi! I'm {}, I don't talk", self.name);
        let voice = format!("Hi!, I don't talk");
        // println!("{}", voice);
        voice
    }
}

impl Animal for Cat {
    fn talk(&self) -> String {
        let voice = format!("Hi! I'm {}, Meow...", self.name);
        // println!("{}", voice);
        voice
    }
}

impl Animal for Dog {
    fn talk(&self) -> String {
        let voice = format!("Hi! I'm {}, Boww Boww!!", self.name);
        // println!("{}", voice);
        voice
    }
}

impl Animal for Duck {
    /*
    Uses the default implementation
    */
}

impl Animal for Horse {
    /*
    Uses the default implementation
    */
}

// Using &impl syntax
fn make_animal_talk_opt1(animal: &impl Animal) {
    println!("This animal says {}", animal.talk());
}

// Using the generic syntax same as above
fn make_animal_talk_opt2<T: Animal>(animal: &T) {
    println!("This animal says {}", animal.talk());
}

// This syntax is also same as the above ones but make more sense
// when dealing with multiple generic types
fn make_animal_talk_opt3<T>(animal: &T)
where
    T: Animal,
{
    println!("This animal says {}", animal.talk())
}

// Traits as return type
// it only works if we retun single type, which means we can not
// return the Cat Type
fn create_any_animal_opt1() -> impl Animal {
    Dog {
        name: "Marly".into(),
    }
}

fn create_any_animal_opt2(cat: bool) -> Box<dyn Animal> {
    if cat {
        let cat = Cat {
            name: "Caty".into(),
        };
        Box::new(cat)
    } else {
        let duck = Duck {
            name: "Flighty".into(),
        };
        Box::new(duck)
    }
}

fn main() {
    let marly = create_any_animal_opt1();
    let björn = Horse {
        name: "Björn".into(),
    };

    make_animal_talk_opt1(&marly);
    make_animal_talk_opt3(&björn);

    let caty = create_any_animal_opt2(true);
    println!("{}", &caty.talk());
    // this will not work; for that the receiving type of the func need to be wraped inside box
    // make_animal_talk_opt2(&caty);
}
