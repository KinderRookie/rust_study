fn main() {
    let mut x = 5; // mutability
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // 불변, 타입 명시, 런타임에만 알 수 있는 값 사용 불가
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    // shadowing
    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");


    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup; // pattern matching

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];

    let nums: [i32; 5] = [1, 2, 3, 4, 5];

    let def = [3; 5]; // [3, 3, 3, 3, 3]

    
}