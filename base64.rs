// Package ID
#![ crate_id = "hellorust#0.1" ]

// Additional metadata attributes
#![ desc = "Hello Rust" ]
#![ license = "GPL" ]
#![ comment = "Just doodling." ]

// Specify the output type
#![ crate_type = "bin" ]

// extern crate stuff

// Mods
mod world {
    use std::string::String;

    pub struct Hello;

    impl Hello {
        pub fn new() -> Hello { Hello }
        pub fn hello(&self) -> String { String::from_str("Hello World") }
        pub fn trello(&self) -> String { self.tello() }
    }

    pub trait Smello {
        fn hello(&self) -> String;
        fn tello(&self) -> String { String::from_str("TELLO") }
    }

    pub trait Fello {
        fn hello(&self) -> String;
        // Multiple methods in scope
        // fn tello(&self) -> String { String::from_str("TELLO") }
    }

    pub trait Bello : Fello {
        // fn hello(&self) -> String; // Multiple methods in scope
        fn bello(&self) -> String;

        fn yello(&self) -> String { String::from_str("YELLO ") + self.hello() }
    }
    
    impl Smello for Hello {
        fn hello(&self) -> String { String::from_str("SMELLO ") + self.hello() }
    }

    impl Fello for Hello {
        fn hello(&self) -> String { String::from_str("FELLO ") + self.hello() }
    }

    impl Bello for Hello {
        fn bello(&self) -> String {
            let fello = self as &Fello;
            String::from_str("BELLO ") + self.hello() + String::from_str(" and ") + fello.hello()
        }
    }
}

fn main() {
    use world::Hello;
    use world::Smello;
    use world::Fello;
    use world::Bello;

    let hello = Hello::new();
    let smello: &Smello = &hello as &Smello;
    let fello: &Fello = &hello as &Fello;
    let bello: &Bello = &hello as &Bello;
    println!("Say: {}", hello.hello())
    println!("Tello: {}", hello.tello())
    println!("Trello: {}", hello.trello())
    println!("Smello: {}", smello.hello())
    println!("Fello: {}", fello.hello())
    println!("Bello hello: {}", bello.hello())
    println!("Bello bello: {}", bello.bello())
    println!("Bello yello: {}", bello.yello())
}
