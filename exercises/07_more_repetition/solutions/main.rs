////////// DO NOT CHANGE BELOW HERE /////////
use std::collections::HashMap;

fn print_hashmap(hashmap: &HashMap<&str, &str>) {
    println!("{hashmap:#?}");
}
////////// DO NOT CHANGE ABOVE HERE /////////

macro_rules! hashmap {
    ( $($k:literal => $v:expr,)* ) => {
        {
            let mut map = HashMap::new();
            $( map.insert($k, $v); )*
            map
        }
    }
}

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    let value = "my_string";
    let my_hashmap = hashmap!(
        "hash" => "map",
        "Key" => value,
    );

    print_hashmap(&my_hashmap);
}
