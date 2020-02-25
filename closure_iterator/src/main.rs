use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

fn main() {
    {
        // Closures
        const LOW_INTENSITY: u32 = 44;
        const HIGH_INTENSITY: u32 = 99;
        let example = |x| x; // one-liner closure
        let s = example(String::from("hello")); // compiler will infer the types of arguments, return values of closure at this point
                                                // let n = example(5);                     so this line will cause an error.
        let annotated = |x: u32| -> u32 { x }; // can annotate the types for verbosity

        struct Cacher<T, K>
        // A struct store closures must be annotated with trait bounds.
        // All closures implement at least one of `Fn, FnMut, FnOnce`. Ordinary functions can implement them too.
        // Generally, when specifying closure trait bounds, one can start with `Fn` only, then compiler will tell if more is needed.
        where
            T: Fn(&K) -> u32,
            K: Eq + Hash + PartialEq,
        {
            calculation: T,       // closure
            map: HashMap<K, u32>, // cache the result, initially `None`, then `Some`
        }
        impl<T, K> Cacher<T, K>
        where
            T: Fn(&K) -> u32,
            K: Eq + Hash + PartialEq,
        {
            fn new(calculation: T) -> Cacher<T, K> {
                Cacher {
                    calculation,
                    map: HashMap::new(),
                }
            }

            fn value(&mut self, arg: K) -> u32 {
                if let Some(v) = self.map.get_mut(&arg) {
                    return *v;
                } else {
                    let v = (self.calculation)(&arg);
                    self.map.insert(arg, v);
                    return v;
                }
            }
        }
        fn gen_workout(key: &str, rand_num: u32) {
            // function where the closure is used.
            let mut expensive_result = Cacher::new(|val| {
                println!("calculating slowly...");
                thread::sleep(Duration::from_secs(2));
                *val
            });
            if key == "low" {
                println!("{} pushups!", expensive_result.value(LOW_INTENSITY));
                println!("{} situps", expensive_result.value(LOW_INTENSITY));
            } else {
                if rand_num == 3 {
                    println!("Break day");
                } else {
                    println!("{} minutes run", expensive_result.value(HIGH_INTENSITY));
                }
            }
        }

        gen_workout("high", 3);
    }
    { // Iterators
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();

        for val in v1_iter { // for loop takes ownership and make v1_iter mutable behind the scenes!
            println!("Got {}", val);
        }
        // Iterator trait only requires `next` method to be implemented.
        let mut v2_iter = v1.iter();
        assert_eq!(v2_iter.next(), Some(&1)); // iterators need to be mutable for calling `next`, and the return value is immutable reference, that is, it iterates over &T.
        let mut v2 = vec![2, 4, 6];
        let v3_iter = v2.iter_mut(); // `iter_mut` is similar with `iter` except it returns mutable reference, that is, it iterates over &mut T.
        for val in v3_iter {
            *val += 3;
        }
        println!("{:#?}", &v2);
        let v4_iter = v2.into_iter(); // `into_iter` will take ownership of v1 and return value is owned values, that is, it iterates over T,
        for mut val in v4_iter {
            val *= 4;
            println!("{}", val);
        }
    }
}
