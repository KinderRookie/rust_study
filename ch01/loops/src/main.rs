fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {result}");

    label_loops();
    println!("\n");
    while_loop();
    println!("\n");
    for_loop();
}

fn label_loops() {

    let mut count = 0;
    'outer: loop {
        println!("count: {count}");

        let mut remaining = 10;

        loop {
            println!("remaining: {remaining}");

            if remaining == 6 {
                break;
            }

            if count == 2 {
                break 'outer;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("Exited the outer loop");
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}


fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("{element}");
    }

    for number in 1..4 {
        println!("{number}");
    }

    

}