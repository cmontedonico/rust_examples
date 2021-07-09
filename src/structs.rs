// Custom datatypes
struct Color {
    red: u8,
    green: u8, 
    blue : u8
}

// tuple struct
struct Color2(u8, u8, u8);

// PErson strcut
struct Person {
    name : String, 
    lastname : String
}

impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person {
            name: first.to_string(), 
            lastname: last.to_string()
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}" , self.name, self.lastname)
    }

    fn set_last_name(&mut self, last: &str){
        self.lastname = last.to_string();
    }

    fn to_tuple(self) -> (String, String) {
        (self.name, self.lastname)
    }
}

pub fn run() {
    let mut c = Color {
        red: 255, 
        green: 0, 
        blue: 0
    };

    c.red = 150;

    println!("Color: {} {} {} ", c.red, c.green, c.blue);


    // Color 2
    let d = Color2(200 , 5,5);
    println!("Color2: {} {} {} ", d.0, d.1, d.2);


    // Person
    let mut p = Person::new("Mary", "Doe");
    println!("Hello {} {}", p.name, p.lastname);
    p.set_last_name("Williams");
    println!("fulllname is  {}", p.full_name());

    println!("person tuple is  {:?}", p.to_tuple());
  
}