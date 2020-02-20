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
        format!("{}, by {} ({})", self.headline, self,author, self.location)
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
pub fn notify(item: impl Summary) {
    println!("{}", item.summarize());
}

// trait bound syntax, more general
pub fn notify_tb<T: Summary>(item1: T, item2: T) {
    println!("blablabla");
}