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

fn main() {
    let marly = Dog {
        name: "Marly".into(),
    };
    let caty = Cat {
        name: "caty".into(),
    };
    let björn = Horse {
        name: "Björn".into(),
    };

    make_animal_talk_opt1(&marly);
    make_animal_talk_opt2(&caty);
    make_animal_talk_opt3(&björn);
}
