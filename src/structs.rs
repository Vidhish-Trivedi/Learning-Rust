// Structs are used to create custom data types.

// Create a Traditional Struct.
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Create a Tuple Struct.
struct Color2(u8, u8, u8);

//////////////////////////////////////////////////////////////////
// Struct with functions associated with it.
struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Construct person.
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }                                                       // Missing ; indicates return statement.
    }

    // Get full name.
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)       // Missing ; indicates return statement.
    }

    // Change attributes of a struct.
    // Set last name.
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to tuple.
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

//////////////////////////////////////////////////////////////////

pub fn run() {
    // Creating an instance of a Traditional Struct.
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    println!("Color: r:{} g:{} b:{}", c.red, c.green, c.blue);

    // Modify values of a struct.
    c.red = 200;
    c.blue = 50;

    println!("Color: r:{} g:{} b:{}", c.red, c.green, c.blue);

    // Creating an instance of a Tuple Struct.
    let mut c2 = Color2(200, 100, 50);

    println!("Color2: r:{} g:{} b:{}", c2.0, c2.1, c2.2);

    // Modify values of a struct.
    c2.0 = 40;
    c2.2 = 120;

    println!("Color2: r:{} g:{} b:{}", c2.0, c2.1, c2.2);
    println!();

    // Creating an instance of a struct using constructors.
    let mut p = Person::new("Jack", "Bauer");
    println!("Person: {} {}", p.first_name, p.last_name);

    p.set_last_name("Williams");

    println!("Full Name: {}", p.full_name());

    println!("Name as a tuple: {:?}", p.to_tuple());
}
