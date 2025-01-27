fn main() {
    println!("Hello, world!");

    if 5 > 4 {
        println!("5 is greater than 4");
    } else {
        println!("5 is not greater than 4");
    }

    let con: bool = false;

    let num = if con { 5 } else { 6 };

    println!("The value of num is: {num}");

    /* 
    if 표현식의 타입이 다르면 안 됨.
    ex) let num = if con { 5 } else { "six" }; -> error
     */

    
}
