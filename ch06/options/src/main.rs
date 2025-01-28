fn main() {
    // Rust has no 'NULL' notion.
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let z = x + y; 서로 다른 타입!
    /*
    바꿔 말하면, T에 대한 연산을 수행하기 전에 Option<T>를 T로 변환해야 합니다.
    이런 방식은 널로 인해 발생하는 가장 흔한 문제인,
    실제로는 널인데 널이 아니라고 가정하는 상황을 발견하는 데 도움이 됩니다.
     */

}

