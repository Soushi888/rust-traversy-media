struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Person {
        Person {
            name,
            age,
        }
    }

    fn who_am_i(&self) {
        println!("Hi ! I am {} and I'm {} years old !", self.name, self.age)
    }
}

trait Transform {
    fn rev(&self) -> String;

    fn output_rev(&self) {
        println!("{}", self.rev());
    }
}


impl Transform for Person {
    fn rev(&self) -> String {
        self.name.chars().rev().collect()
    }
}

pub fn run() {
    let p = Person::new("Sacha".parse().unwrap(), 28);
    p.who_am_i();

    println!("{}", p.name);
    p.output_rev();
}
