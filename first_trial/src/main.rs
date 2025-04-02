use static_cell::StaticCell;
use std::collections::HashMap;
use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref memoize: Mutex<HashMap<u32, u32>> = {
        let m = HashMap::new();

        Mutex::new(m)
    };
}

static SOME_MAP: StaticCell<HashMap<u32, u32>> = StaticCell::new();

fn fibonacci(value: u32) -> u32 {
    let mut map = memoize.lock().unwrap();

    if map.contains_key(&value) {
        return *map.get(&value).unwrap();
    }

    if value == 1 || value == 2 {
        return 1;
    }

    map.insert(value, fibonacci(value - 1) + fibonacci(value - 2));

    return *map.get(&value).unwrap();
}

fn main() {
    println!("Hello, world!");

    println!("{:?}", fibonacci(100));
}
