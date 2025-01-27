fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // &s1: s1의 참조자
    // 값에 대해 참조는 하지만 소유는 하지 않는다. 따라서 소유권을 이동하지 않는다.

    println!("The length of '{s1}' is {len}");


    // 가변 참조자
    let mut s = String::from("hello");

    println!("{s}");

    change(&mut s);

    println!("{s}");

    // 가변 참조자는 여러 개를 만들 수 없다.
    // let r1 = &mut s;
    // let r2 = &mut s; // error

    
}



fn calculate_length(s: &String) -> usize {
    s.len()

    // 참조자를 이용하여 값을 변경할 수 없다.
    // s.push_str(", world"); // error
}

fn change(s: &mut String) {
    s.push_str(", world");
}