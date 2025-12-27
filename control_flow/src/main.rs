fn main() {
    println!("{}", color_to_number("red"));
    
    println!("{}", factorial_iterative(5));
    println!("{}", factorial_recursion(5));
}
fn color_to_number(color: &str) -> i8 {
    // if color == "red" {
    //     1
    // } else if color == "green" {
    //     2
    // } else if color == "blue" {
    //     3
    // } else {
    //     0
    // }

    match color {
        "red" => 1,
        "green" => 2,
        "blue" => 3,
        _ => 0,
    }
}

fn factorial_iterative(number: i64) -> i64 {
    let mut product = 1;
    let mut count = number;
    while count > 0 {
        product *= count;
        count -= 1;
    }
    product
}

fn factorial_recursion(number: i64) -> i64 {
    if number == 1 {
        return 1;
    }
    number * factorial_recursion(number - 1)
}
