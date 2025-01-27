fn main() {
    println!("Hello, world!");

    function(5, 'a');

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    let x = plus_one(5);

    println!("The value of x is: {x}");

}


fn function(x: i32, y: char) {
    println!("The value of num is: {x}");
    println!("The value of char is: {y}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}