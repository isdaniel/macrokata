////////// DO NOT CHANGE BELOW HERE /////////
fn print_result(num: i32) {
    println!("The result is {num}");
}
////////// DO NOT CHANGE ABOVE HERE /////////

macro_rules! math {
    ($val1:literal plus $val2:literal) => {
        $val1 + $val2
    };

    (square $val1:literal) => {
        $val1 * $val1
    };
}

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    print_result(math!(3 plus 5));
    print_result(math!(square 2));
}
