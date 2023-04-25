use std::io;

fn main() {
    let t = true;

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let five = tup.0;

    // println!("The value of y is: {five}");

    let a: [i32; 5] = [1,2,3,4,5];

    let firtst = a[0];



    // Array index
    
    let a = [1,2,3,4,5];

    println!("Please enter an array index:");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");


    
}
