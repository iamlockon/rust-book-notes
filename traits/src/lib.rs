use std::fmt::{ Display, Debug };

// One can't implement traits on a type if none of the trait or type is local to the crate.
// That is to say, we can't implement external traits on external types.
// This 'orphan rule' is to avoid two crates implement the same trait for the same type.
pub trait Summary {
    fn summarize(&self) -> String; // required
    fn platform(&self) -> String {
        format!("{}", self.summarize()) // optional with default behavior, can call other methods.
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// syntax sugar for trait bound, convenient
pub fn notify(item: impl Summary + Display) -> () { // use `+` for multiple trait bounds
    println!("{}", item.summarize());
}

// trait bound syntax, more general
pub fn notify_2<T: Summary + Display, U: Debug>(item1: T, item2: U) -> () {
    println!("blablabla");
}
// the above one is equivalent to:
pub fn notify_3<T, U>(item1: T, item2: U) -> ()
    where T: Summary + Display, // use `where` for shorter function signature
          U: Debug
{
    ()
}

// This won't compile, `impl Summary` cannot return different concrete types
// fn return_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from("Title"),
//             location: String::from("PA"),
//             author: String::from("Ice"),
//             content: String::from("ccc"),
//         }
//     } else {
//         Tweet {
//             username: String::from("Icc"),
//             content: String::from("cccc"),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

// Conditionally implement methods for structs on a generic type depending on trait bounds
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x:T, y:T) -> Self { // Pair always implement `new`
        Self {
            x,
            y
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) { // Pair only implement cmp_display if T implements PartialOrd and Display
        if self.x >= self.y {
            println!("Largest member is x = {}", self.x);
        } else {
            println!("Largest member is y = {}", self.y);
        }
    }
}
// We can also do the same for trait implementations(ex: ToString)
// impl<T: Display> ToString for T {
//  
// }
// And that's why we can write:
// let s = 3.to_string();