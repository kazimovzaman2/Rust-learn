fn five() -> i32 {
    return 5;
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    println!("Hello, world!");

    print_labeled_measurement(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };

    println!("{y}");

    let x = five();
    println!("{x}");

    let x = add_one(5);
    println!("{x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
