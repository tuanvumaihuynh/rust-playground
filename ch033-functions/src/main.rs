fn main() {
    println!("Hello, world!");

    another_function(5);

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y: {y}");

    let x = five();
    println!("The value of x: {x}");
}

fn another_function(x: i32) {
    println!("Another function: {x}")
}

fn five() -> i32 {
    5
}
