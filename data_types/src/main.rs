fn main() {
    let distance: i32 = 1_337;
    let miles: i16 = distance as i16;

    let height = 3.436157;
    println!("the float value is {height:.3}");

    let with_milk = false;
    let with_sugre = true;

    let is_my_type_of_coffe = with_milk && with_sugre;
    let _is_acceptable_coffe = with_milk || with_sugre;

    let price: [i8; 4] = [13, 23, 75, 100];
    println!("{:#?}", price);

    let combo: (i16, f64, bool, [i8; 4]) = (miles, height, is_my_type_of_coffe, price);
    dbg!(combo);
}
