fn main() {

    let width = 30;
    let height = 50;

    let rect1 = (30, 50);
    let rect2 = Rectangle { width, height };

    // println!(
    //     "The area of rectangle is {}",
    //     area(width, height)
    // );

    println!(
        "The area of rectangle is {}",
        area(&rect2)
    );

    // println!("The width and height is {}", rect2);
    /*
    Error: `Rectangle` doesn't implement `std::fmt::Display`
    Did not implemented Display trait
     */
    println!("The width and height is {:?}", rect2); // Debug mode
    // Prettier: {:#?}


    // dbg! macro
    // 표현식의 소유권을 가져와서, 결과값과 코드 라인을 출력하고 소유권 반환
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    // dbg!(rect2); // 오류...
    /*
    caller 에게 소유권 다시 넘겨준다.
    근데 caller 없다.
    dbg! 가 맛있게 먹었답니다.
     */
    dbg!(&rect2);


    // method in structure
    println!("The area of rectangle is {}", rect2.area());

    if rect2.width() {
        println!("The rect2's width is {}", rect2.width);
    }


    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(30); // 연관 함수



}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area(dimension: (u32, u32)) -> u32 {
//     dimension.0 * dimension.1
// }


fn area (rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[derive(Debug)] // annotation for Debug trait <- already implemented.
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    /*
     동작하는 데 해당 타입의 인스턴스가 필요하지 않다면
     self를 첫 매개변수로 갖지 않는 (따라서 메서드가 아닌)
     연관 함수를 정의할 수도 있습니다.
     */
    fn square(size: u32) -> Self { // Self == Rectangle (impl 뒤에 적힌 타입)
        Self {
            width: size,
            height: size,
        }
    }


}