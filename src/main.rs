struct User {
    active: User,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // declaring variables are declared by the syntax let i:i128 = 123021344535; where i=> signed integer and 128 => no. of bits in the number, by default is i32 (Better Memory Reservation for a variable)
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

    // Memory Management
    // calling the heap and stack functions
    stack();
    heap();
    println!(
        "\nThis is the example for the update string {}",
        update_string("hello my name is ".to_owned())
    );

    // ownership

    let s1 = String::from("hello this is the string");
    let s2 = s1;
    println!("\nThis is the string s2: {}", s2);

    // println!("S1 is no longer accessible: {}", s1); check ownership.png in ./assets/ whenever the owner goes out of scope the data die   s.
    //   THE ERROR:  borrow of moved value: `s1`
    // value borrowed here after moverustcClick for full compiler diagnostic
    // macros.rs(143, 28): Error originated from macro call here
    // main.rs(59, 5): Error originated from macro call here
    // main.rs(55, 14): value moved here
    // main.rs(53, 9): move occurs because `s1` has type `String`, which does not implement the `Copy` trait
    // main.rs(55, 16): consider cloning the value if the performance cost is acceptable: `.clone()`

    let rihanna = String::from("hellow");

    // this_fn_takes_ownership(rihanna);
    // println!("{}", rihanna);  //this cannot be done because the function "this_fn_takes_ownership" actually takes the ownership

    // let new_string = this_fn_returns_after_use(rihanna);
    // println!(
    //     "This is the value of Rihanna after she came back: {}",
    //     new_string
    // );

    //Borowing

    let addr = &rihanna;

    borrow_variable(addr);

    borrow_variable(addr);

    println!("{}", rihanna); //no compilation error because the variable was borrowed by passing the address to string than the string itself to the borrow_variable function.

    // let mut changeable_string = String::from("this is the modified hello");
    // hanky_panky(&mut changeable_string);
}

// declaring function in rust,
// fn function_name_here(parameter1: i32, parameter2: String) -> i32{
// function logic here
// return value;
// }
// heap vs stack, stack stores predicitable variables such as fixed sized arrays, nyumbers etc., in heap strings, vectors are actually stored.

fn get_sum(a: i32, b: i32) -> i32 {
    return a + b;
}

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

    // pushing to the string 100 times to see the change in pointer
    // for _ in 0..100 {
    //     s.push_str(" Shubham Gaur ");

    //     println!(
    //         "\nAfter Change, Capacity is: {}, Lenght is: {}, pointer is: {:p}",
    //         s.capacity(),
    //         s.len(),
    //         s.as_ptr()
    //     );
    // }

    s.push_str(" Shubham Gaur ");
    println!(
        "\nAfter Change, Capacity is: {}, Lenght is: {}, pointer is: {:p}",
        s.capacity(),
        s.len(),
        s.as_ptr()
    );
    return s;
}

// fn this_fn_takes_ownership(some_string: String) {
//     println!("{}", some_string);
// }

// fn this_fn_returns_after_use(some_string: String) -> String {
//     //keeping it casual
//     println!("\n{}", some_string);
//     return some_string;
// }

fn borrow_variable(some_string: &String) {
    println!("\nThis string is borrowed here: {}", some_string);
}

// hanky-panky is not allowed with borrowed variables
// fn hanky_panky(some_string: &mut String) {
//     some_string.push_str("world");
//     return some_string;
// }
