fn main() {
    
    {
    let s1 = String::from("hello");
    let s2 = s1; // move

    // println!("{s1}"); // error

        /*
    stack 같이 컴파일 시점에 크기가 결정되는 데이터는 copy
    정확히는, 타입에 Copy 트레이트가 구현되어 있어야 함.
    ex)
    let x = 5;
    let y = x;
    
    println!("{x}"); // ok
    
    heap 같이 런타임 시점에 크기가 결정되는 데이터는 move
    타입에 [Drop] 트레이트가 구현되어 있음.
     */


    }

    let s1 = String::from("hello");

    take_ownership(s1);

    println!("{s1}"); // error
    // 소유권이 이동되었기 때문에 s1을 사용할 수 없음.

    



}


fn take_ownership(s: String) {
    println!("{s}");
}