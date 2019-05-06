mod sound {
    pub mod instrument {
        pub fn clarinet() {
            // function body
        }
    }
}

mod performance_group {
    // this brings instrument into scope for outside code calling
    // performance_group
    pub use crate::sound::instrument;

    pub fn clarinet_trio() {
        instrument::clarinet();
        instrument::clarinet();
        instrument::clarinet();
    }
}

// this statement means you don't have to
// specify the path of the `instrument` module all the time
// because it brings this module into scope
use crate::sound::instrument;

// or you can bring it into scope with the relative path
//use self::sound::instrument;

// or super::sound::instrument if it's rooted in the parent module

// but for structs and enums, specifying full path to the item is 
// correct, instead of just the parent module's path
use std::collections::HashMap;

#[allow(dead_code)]
mod plant {
    // the struct is public
    pub struct Vegetable {
        pub name: String, // this field is public
        id:  i32, // this field is not public
    }

    // this Constructor is necessary because Vegetable has 
    // private fields
    impl Vegetable {
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1,
            }
        }
    }
}

mod menu {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

#[allow(unused_variables)]
fn main() {
    let mut v = plant::Vegetable::new("squash");

    v.name = String::from("butternut squash");
    println!("{} are delicious", v.name);

    // this won't work because we can't access the id field
    // println!("The id is {}", v.id);
    
    let order1 = menu::Appetizer::Soup;
    let order2 = menu::Appetizer::Salad;

    // we don't want to bring clarinet into scope, just its parent
    // module, to make it clear that this isn't a locally defined
    // function
    instrument::clarinet();

    let mut map = HashMap::new();
    map.insert(1, 2);

    // this is allowed because `clarinet_trio()` is a pub function
    performance_group::clarinet_trio();

    // this is allowed because of `pub use` in the performance_group module
    performance_group::instrument::clarinet();
}
