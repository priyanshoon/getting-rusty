fn main() {
    let mut x: u16 = 32;
    println!("{x}");
    x = 323;
    println!("{x}");
    println!("Hello, world!");
    println!("Hello Priyanshu!");
    another_func(34, 'c');
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is {y}");
    let x = five();
    println!("The value of x is : {x}");
    let result = plus_one(34);
    println!("the value of result : {result}");

    let mut number = 3;

    if number < 5 {
        println!("the number is less than 3");
    } else {
        println!("the number is greater than 3");
    }

    if number != 0 {
        println!("The number is not 3");
    }

    number = 6;

    if number % 4 == 0{
        println!("The number is divisible by four");
    } else if number % 2 == 0{
        println!("The number is divisible by two");
    } else if number % 3 == 0 {
        println!("The number is divisible by three");
    } else {
        println!("The number is not divisible");
    }

    let mut counter = 0;
    let result_loop = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The value of result_loop is {result_loop}");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is : {element}");
    }
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn five() -> i32 {
    5
}

fn another_func(x: i32, unit_lable: char) {
    println!("Hello from another function {x} {unit_lable}");
}
