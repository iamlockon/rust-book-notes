fn main() {
    // Vector
    {
        {
            // Init, push, get
            let v: Vec<i32> = Vec::new(); // empty vector, note that type annotation is needed.
            let mut v = vec![1, 2, 3]; // use `vec!` for vector with initial values.
            v.push(5);
            match v.pop() {
                Some(v) => println!("Pop {}", v),
                None => println!("Nothing to pop!"),
            }
            let two = &v[1]; // immutable borrow
                             // `v.push(7)` or any operation that mutate the vector will fail compilation at this point.
            match v.get(1) {
                Some(two) => println!("Get {}", two),
                None => println!("Nothing at index 1"),
            }
        }

        {
            // Iterating
            let v = vec![123, 23, 23];
            for i in &v {
                // immutable
                println!("iterating:");
                println!("{}", i);
            }
            let mut vv = vec![333, 55, 66];
            for i in &mut vv {
                // mutable
                *i += 3;
            }
        }

        {
            // Store pre-defined objects in vector (if not known, use `trait`)
            enum ProductColumn {
                Name(String),
                Price(f64),
                Type(i32),
            }
            let row = vec![
                ProductColumn::Name(String::from("PS7")),
                ProductColumn::Price(32.99),
                ProductColumn::Type(4),
            ];
        }
    }
    // String
    {
        {
            // Init, update, concat, get
            let s = String::new();
            let ss = "literal".to_string();
            let sss = String::from("literal");
            let mut mut_s = String::from("foo");
            mut_s.push_str("bar");
            println!("Push bar to foo: {}", mut_s);
            mut_s.push('d');

            let s1 = String::from("tic");
            let s2 = String::from("tac");
            let s3 = String::from("toe");
            let concat_s = format!("{}={}={}", s1, s2, s3);

            // use `add` method internally, `fn add(self, s: &str) -> String {}`
            // so `s1` will be gone after this line.
            let own_cat_s = s1 + &s2;

            // indexing String will not compile
            // let h = ownCatS[2];
            // Strings are Vec<u8> internally.
            let he = "Здравствуйте";
            let s = &he[0..6]; // slicing with invalid char boundary will panic and crush the program
                               // let unsafeS = &he[0..3];
        }

        {
            // Iterating
            let d = "data";
            for c in d.chars() {
                println!("{}", c);
            }
            let s = String::from("中文and");
            for c in s.chars() {
                println!("{}", c);
            }

            for b in s.bytes() {
                println!("{}", b);
            }
        }
    }
    // Hash map
    {
        use std::collections::HashMap;
        { // Init, update
            let mut scores = HashMap::new();
            scores.insert(String::from("Blue"), 10);
            scores.insert(String::from("Yellow"), 202); // all keys and values should have the same type, respectively
            
            let teams = vec![String::from("Blue"), String::from("Yellow")];
            let init_scores = vec![10, 202];
            let mut same_scores: HashMap<_, _> = teams.iter().zip(init_scores.iter()).collect();
            let s = String::from("Fav Number");
            let v = 10;
            same_scores.insert(&s, &v);
            println!("{}", s);
            let blue_score = same_scores.get(&String::from("Blue"));
            same_scores.entry(String::from("Blue")).or_insert(33); // overwrite on absence, noop on existence
        }

        { // Iterating
            let mut map = HashMap::new();
            map.insert("AAA", 10);
            map.insert("AAB", 10);
            for (key, value) in map {
                println!("{}: {}", key, value);
            }
        }
    }
}
