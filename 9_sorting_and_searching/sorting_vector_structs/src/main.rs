#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Person {
            name,
            age
        }
    }
}

fn main() {
    let mut people = vec![
        Person::new("Tyrion".to_string(), 25),
        Person::new("Cersei".to_string(), 30),
        Person::new("Jaime".to_string(), 31),
    ];

    // Sort by derived natural order (Name and age)
    people.sort();

    assert_eq!(
        people,
        vec![
            Person::new("Cersei".to_string(), 30),
            Person::new("Jaime".to_string(), 31),
            Person::new("Tyrion".to_string(), 25),
        ]);

    // Sort by age
    people.sort_by(|a, b| b.age.cmp(&a.age));

    assert_eq!(
        people,
        vec![
            Person::new("Jaime".to_string(), 31),
            Person::new("Cersei".to_string(), 30),
            Person::new("Tyrion".to_string(), 25),
        ]);

}

