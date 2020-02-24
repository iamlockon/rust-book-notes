#[cfg(test)]
mod tests {
    #[test] // attribute indicates this is a test function
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    use super::*;
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height:7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { width: 8, height:7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(!smaller.can_hold(&larger),
            "Actual results: {}", smaller.can_hold(&larger) // can use format string to print desired information when failed.
        );
    }

    #[test]
    fn result_struct() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 != 4, sth must be wrong.."))
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// some special attributes:
// #[should_panic] : test should panic to pass
// #[ignore] : Exclude this test, add under `#[test]`


// Run tests with matched name:
// $cargo test <pattern>
// Run tests with matched module:
// $cargo test <module>::<pattern>
// Do not capture stdout for passed tests
// $cargo test -- --nocapture
// Run ignored tests
// $cargo test -- --ignored

// integration test files will be placed in `tests` directory
// and one can run all tests in one particular file with:
// $cargo test --test <filename>

// NOTE:
// If our project is a binary crate that only contains a src/main.rs file and doesn’t have a src/lib.rs file,
// we can’t create integration tests in the tests directory and bring functions defined in the src/main.rs file into scope with a use statement.
// Only library crates expose functions that other crates can use; binary crates are meant to be run on their own.