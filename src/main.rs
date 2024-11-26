fn main() {
    let x = 10;
    let y = 20;
    println!("{},{}", y, x);
}

#[test]
fn imutable() {
    let apple = "apple";
    println!("{}", apple);
}

#[test]
fn mutable() {
    let mut apple = "apple";
    println!("{}", apple);
    apple = "orange";
    println!("{}", apple);
}

#[test]
fn static_variable() {
    let mut apple = "apple";
    // APPLE = 2;
    println!("{}", apple);
    apple = "orange";
    println!("{}", apple);
}

#[test]
fn shadowing() {
    let apple = "apple";
    println!("{}", apple);
    let apple = 10;
    println!("{}", apple);
}

#[test]
fn scalar_data_types() {
    let x = 10;
    let y = 10.0;
    let z = true;
    let a = 'a';
    println!("x: {}, y: {}, z: {}, a: {}", x, y, z, a);
}

#[test]
fn oprations() {
    let x = 10;
    let y = 20;
    let z = x + y;
    let a = x - y;
    let b = x * y;
    let c = x / y;
    let d = x % y;
    println!("z: {}, a: {}, b: {}, c: {}, d: {}", z, a, b, c, d);
}

#[test]
fn tuppel() {
    let tup = (500, 6.4, 1, true);
    println!("x: {:?}", tup.0);
    let (x, y, z, a) = tup;
    println!("x: {}, y: {}, z: {}, a: {}", x, y, z, a);
    let (d, s, _, _) = tup;
    println!("d: {}, s: {}", d, s);
}

#[test]
fn muttuo() {
    let mut data:(i32, f64, u8) = (500, 6.4, 1);
    println!("data: {:?}", data);
    let (a, b, c) = data;
    println!("a: {}, b: {}, c: {}", a, b, c);

    data.0 = 100;
}


fn unit(){
    println!("unit");
}

#[test]
fn testUnit(){
    let result = unit();
    println!("result: {:?}", result);
}


#[test]
fn array(){
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("arr: {:?}", arr);
    arr[0] = 10;
    println!("arr: {:?}", arr[0]);
    println!("arr: {:?}", arr.len());
}