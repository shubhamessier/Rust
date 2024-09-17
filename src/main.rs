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
        get_sum(23412, 123)
    );

    // calling the heap and stack functions

    stack();

    heap();

    println!(
        "\nThis is the example for the update string {}",
        update_string("hello my name is ".to_owned())
    )

    // Memory Management
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

// heap vs stack, stack stores predicitable variables such as fixed sized arrays, nyumbers etc., in heap strings, vectors are actually stored.

fn stack() {
    let a: i32 = 10;
    let b: i32 = 50;

    let c: i32 = a + b;

    println!("\nStack fn: The Sum of {} and {} is {}", a, b, c);
}

fn heap() {
    let data = String::from("10101011001100010101");
    let data2 = String::from("101001011001111000011");

    let combined = format!("{}{}", data, data2);

    println!("\nHeap fn: Combined string is '{}'", combined);
}

fn update_string(mut s: String) -> String {
    // let mut s = String::from("Initial string");
    println!("\nBefore update the string is: {}", s);

    s.push_str(" Shubham Gaur");

    return s;
}
