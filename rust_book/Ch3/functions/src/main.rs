//functions/src/main.rs

fn main() {
    println!("Hello, world!");

    another_function1();

    another_function2(5);

    print_labeled_measurement(5, 'h');

    let y = 6;

    println!("The value of y is: {y}");

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    let x = five();

    println!("The value of x is: {x}");

    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn another_function1() {
    println!("Another function.");
}

fn another_function2(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

//This function will throw an error because it does not return a value
// fn plus_one_error(x: i32) -> i32 {
//     x + 1;
// }
