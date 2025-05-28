////////// DO NOT CHANGE BELOW HERE /////////
use std::collections::HashMap;

fn print_hashmap(hashmap: &HashMap<&str, &str>) {
    println!("{hashmap:#?}");
}
////////// DO NOT CHANGE ABOVE HERE /////////
//$( hm.insert($k, $v); )*
macro_rules! hashmap {
    ($($key:literal => $value:expr,)*) => {
        {
            let mut map = HashMap::new();
            $( map.insert($key,$value); )*
            map
        }
    };
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
