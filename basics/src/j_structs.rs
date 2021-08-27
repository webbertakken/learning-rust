// Silence some warnings so they don't distract from the exercise.
#![allow(unused_variables)]

pub fn main() {
    basic_struct();
}

fn basic_struct() {
    struct RedFox {
        enemy: bool,
        life: u8,
    }

    // Default instantiating is quite verbose
    let fox = RedFox {
        enemy: true,
        life: 70,
    };

    impl RedFox {
        // new is the convention for creating a new struct with default values.
        fn new() -> Self {
            Self {
                enemy: true,
                life: 70,
            }
        }

        // associated function
        fn function() {
            // do something
        }

        // methods
        fn walk(self) {
            // do something
        }

        fn borrow(&self) {
            // do something
        }

        fn mutable_borrow(&mut self) {
            // do something
        }
    }

    // Instantiate using implementation ✅
    let mut fox = RedFox::new();

    let life = fox.life;

    fox.enemy = false;
    fox.walk();
}

