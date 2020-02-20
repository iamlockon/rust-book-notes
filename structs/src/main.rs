use std::time;

#[derive(Debug)]
enum ItemType { // enum can have different types and amount of associated data
    FOOD(String),
    TOOL { name: String, power: u32 }, // anonymous struct
    BOOK(String, String),
    ELECTRONIC(u32)
}

#[derive(Debug)]
struct Inventory {
    item_count: u32,
    item_name: String, // the struct owns the String.
    item_type: ItemType,
    last_stock: time::SystemTime
}

impl Inventory { // Add methods for this struct, can have multiple `impl` blocks.
    // For struct method, first parameter always `self`. 
    // When we want to modify instances(mutable borrow), use `&mut self,
    // When reading(immutable borrow), use `&self`,
    // When consuming(take ownership), use `self`.
    fn edit_stock(&mut self, amount: i32) -> u32 { 
        self.item_count = if amount >= 0 {
            self.item_count + amount as u32 // leave out semicolon for return expression
        } else {
            self.item_count - (-amount) as u32 // leave out semicolon for return expression
        };
        self.last_stock = time::SystemTime::now();
        self.item_count
    }
    fn get_stock_info(&self) -> (u32, &str, &ItemType, &time::SystemTime)  {
        (self.item_count, &self.item_name, &self.item_type, &self.last_stock)
    }
    // For associated functions(static method), do not have `self`
    fn create_inventory(count: u32, name: &str, r#type: ItemType) -> Inventory {
        Inventory { item_count: count, item_name: String::from(name), item_type: r#type, last_stock: time::SystemTime::now() }
    }
}

struct Point(i8, i8, String);

fn main() {
    let product_a = Inventory {
        item_count: 23,
        item_name: String::from("A"),
        item_type: ItemType::BOOK(String::from("Mystery"), String::from("English")),
        last_stock: time::SystemTime::now()
    };
    let mut product_b = Inventory {
        item_name: String::from("B"), // except this field, all are updated by product_a (unlike in JS)
        item_type: ItemType::FOOD(String::from("Fish")),
        ..product_a
    };
    println!("Print with :? format => {:?}", product_b); // `:?` tell println! we want the output format `Debug`, and we should add `#[derive(Debug)]` before the struct definition 
    println!("Print with :#? (pretty print) format => {:#?}", product_b);
    // Tuple struct
    let home = Point(2, 5, String::from("Home"));
    println!("{} {} {}", home.0, home.1, home.2);

    let Point(x, y, name) = home;
    println!("{} {} {}", x, y, name);

    let rem = product_b.edit_stock(-3);
    println!("{} left.", rem);
    let (_stock, _name, _type, _last) = product_a.get_stock_info();

    let product_c = Inventory::create_inventory(23, "C", ItemType::ELECTRONIC(4444));
    println!("{:?}", product_c);
    let product_d = Inventory::create_inventory(23, "D", ItemType::TOOL{ name: String::from("Screwdriver"), power: 330 });
    println!("{:?}", product_d);

    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("Three"),
        _ => (),
    }

    let mut ctr = 0;
    let t = ItemType::BOOK(String::from("Fish"), String::from("Beef"));
    match t {
        ItemType::FOOD(_val) => println!("Food!"),
        _ => ctr += 1
    }
    // the above is equivalent to:
    let t2 = ItemType::BOOK(String::from("Fish"), String::from("Beef"));
    if let ItemType::FOOD(_val) = t2 {
        println!("Food!!");
    } else {
        ctr += 1;
    }
    println!("{}", ctr); // this line is needed or will be warned that `ctr` is never read.
}
