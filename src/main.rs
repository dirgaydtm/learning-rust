fn main() {
    println!("Hello, world!");
}

#[test]
fn test_1() {
    println!("Hello, my nigga!");
}

#[test]
fn test_2() {
    let name = "dirga yuditama";
    print!("hello, {}", name);
}

#[test]
fn test_3() {
    let mut name = "dirga yuditama";
    print!("hello, {}", name);

    name = "mamang";
    print!("Hello {}\n", name)
}

#[test]
fn test_4() {
    let name = "dirga yuditama";
    print!("hello, {}\n", name);

    let name = 3;
    print!("{}\n", name)
}

#[test]
fn test_5() {
    let age: i32 = 4;
    print!("{}", age);

    let age: i32 = -4;
    print!("{}", age);

    let age: u32 = 4;
    print!("{}", age);

    let decimal: f32 = 3.4;
    print!("{}", decimal)
}

#[test]
fn test_6() {
    let age: i32 = 4;
    print!("{}", age);

    let age2: i16 = age as i16;
    print!("{}", age2);

    let age3: i8 = age2 as i8;
    print!("{}", age3);

    // integer oveflow
    let number  = 100000000;

    let number2: i8 = number as i8;
    print!("{}", number2);
}


