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

    // 그냥 참조자는 여러 개 가능
    // 불변과 가변 참조자 혼용은 안 됨.
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    // let r3 = &mut s; // 오류! 유발 지점
    let r3 = &s;

    println!("{r1}, {r2}, {r3}"); // 오류 발생 지점
    // 어떤 값에 대한 불변 참조자가 있는 동안, 가변 참조자를 만들 수 없다.

    {
        let mut s = String::from("hello");

        let r1 = &s;
        let r2 = &s;

        println!("{r1}, {r2}");

        let r3 = &mut s;
        println!("{r3}")
    } // 근데 이 코드는 그냥 실행된다
    // 왜냐면 r1, r2는 사용 안되는데, 참조자는 정의된 시점부터 해당 참조자가
    // 마지막으로 사용된 부분까지 유효하다.

    // let reference_nothing = dangle();
    let reference_nothing = not_dangle();

}


fn dangle() -> &String { // String의 참조자를 반환 함
    let s = String::from("hello");

    &s // 참조자 반환

} // s가 스코프 밖으로 벗어나고 버려집니다. 메모리 해제 됨
// 값이 버려진 참조자를 반환 하므로 에러 발생!

fn not_dangle() -> String {
    let s = String::from("hello");

    s
}

fn calculate_length(s: &String) -> usize {
    s.len()

    // 참조자를 이용하여 값을 변경할 수 없다.
    // s.push_str(", world"); // error
}

fn change(s: &mut String) {
    s.push_str(", world");
}