
#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String
}


impl Person {
    fn new(first: &str, name: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: name.to_string()
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }   

    fn set_last_name(&mut self, name: &str) {
        self.last_name = name.to_string();
    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}


#[derive(Debug)]
struct A <'a> {
    s: &'a str
}

fn main() {
    let mut p = Person::new("John","Snow");
    println!("Stark: {} {}", p.first_name,p.last_name);

    p.set_last_name("Stark");
    println!("Stark: {} {}", p.first_name,p.last_name);

    println!("{:?}", p.to_tuple()); // p has been moved

    // example using lifetime
    let s = "hello there".to_string();
    let a = A { s: &s };

    println!("{:?}", a);
}


