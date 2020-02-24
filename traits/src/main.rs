mod lib; // load lib.rs
use crate::lib::Summary;

fn largest_copy<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() { // item reference is destructured, making type of item T
        if item > largest { // compare Ts
            largest = item;
        }
    }

    largest
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() { // type of item &T
        if item > largest { // compare &T is possible since references to types implementing PartialOrd also implement PartialOrd.
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 56, 23, 66, 77];

    let result = largest(&number_list);
    println!("Largest number {}", result);

    let char_list = vec!['y', 'b', 'a', 'q'];
    let result = largest_copy(&char_list);
    println!("Largest char {}", result);
    
    let news = lib::NewsArticle {
        headline: String::from("Title"),
        location: String::from("PA"),
        author: String::from("Ice"),
        content: String::from("ccc"),
    };

    println!("{}", news.summarize());

        
}
