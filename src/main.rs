fn main() {
    // variables are declared by the syntax let i:i128 = 123021344535; where i=> signed integer and 128 => no. of bits in the number, by default is i32 (Better Memory Reservation for a variable)
    // types are: i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64, bool, char

    let x: i8 = -125;
    let y: i8 = 15;

    // booleans
    let is_male = true;
    let is_above_18 = true;

    // if-else
    if is_male {
        println!("\nYou're a male!");
    } else {
        println!("You're not a male!");
    }

    if is_above_18 {
        println!("\nYou're an legal male!");
    }

    println!("\nThe value of x: {}, The value of y: {}", x, y);

    let name_string = String::from("Ramiro Thunder");

    let char1 = name_string.chars().nth(4);

    println!("\nThe name for the string is: {}", name_string);

    println!("\nThis is the character {}", char1.unwrap());

    println!(
        "\nThis is the Sum of the Two arbitary numbers here: {}",
        get_sum(234, 123)
    );
}

// ownership
fn get_sum(a: i32, b: i32) -> i32 {
    return a + b;
}

// declaring a function in rust,

// fn function_name_here(parameter1: i32, parameter2: String) -> i32{
// function logic here
// return value;
// }
