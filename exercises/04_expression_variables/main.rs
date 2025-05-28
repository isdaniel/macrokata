////////// DO NOT CHANGE BELOW HERE /////////
fn print_result(num: i32) {
    println!("The result is {num}");
}
////////// DO NOT CHANGE ABOVE HERE /////////

macro_rules! math{
    (square $x:expr) => {
        $x * $x
    };
    ($x:expr, plus, $y:expr) => {
        $x + $y
    };
}

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    let var = 5;
    print_result(math!((2 * 3), plus, var));
    print_result(math!(square var));
}
