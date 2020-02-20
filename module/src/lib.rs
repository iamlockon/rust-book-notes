// In one package, there can at most exist one library module. But multiple binary/application packages are allowed.
// To access something, the whole path needs to be accessible.
// By default, all childs in a parent, all members in a struct are private, but all variants in enums are public.

mod front_of_house; // using `;` after a module tells Rust to load the contents from another file with the same name.

// `pub use` can re-export the import, so external code can use `hosting::add_to_waitlist()` as well.
// Most importantly, this will decouple the internal module structure with exposed API
pub use crate::front_of_house::hosting; 

pub fn eat_at_restaurant() { // part of public API of this package
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist(); // `crate` denotes the root of module tree

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // `use` can shorten path
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");

    let order1 = back_of_house::Appetizer::Soup;
}

fn serve_order() {}

mod back_of_house {
    pub enum Appetizer { // All variants are public
        Soup,
        Salad,
    }
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String, // private field
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_order() {
        cook_order();
        super::serve_order(); // use `super` for accessing parent module, in this case `crate` (crate root)
    }
    fn cook_order() {}
}