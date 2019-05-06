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
}
