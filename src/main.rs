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
    let mut data: (i32, f64, u8) = (500, 6.4, 1);
    println!("data: {:?}", data);
    let (a, b, c) = data;
    println!("a: {}, b: {}, c: {}", a, b, c);

    data.0 = 100;
}

fn unit() {
    println!("unit");
}

#[test]
fn testUnit() {
    let result = unit();
    println!("result: {:?}", result);
}

#[test]
fn array() {
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("arr: {:?}", arr);
    arr[0] = 10;
    println!("arr: {:?}", arr[0]);
    println!("arr: {:?}", arr.len());
}

#[test]
fn two_d_array() {
    let matix = [[1, 2], [3, 4]];

    println!(
        "{:?}, {:?}\n{:?}, {:?}",
        matix[0][0], matix[0][1], matix[1][0], matix[1][1]
    );

    let det = matix[0][0] * matix[1][1] - matix[0][1] * matix[1][0];
    println!("det: {:?}", det);

    let adj = [[matix[1][1], -matix[0][1]], [-matix[1][0], matix[0][0]]];
    let a_invers = 1 / det;
    println!("{:?}", a_invers);
    let inv = [
        [adj[0][0] * a_invers, adj[0][1] * a_invers],
        [adj[1][0] * a_invers, adj[1][1] * a_invers],
    ];
    println!("inv: {:?}", inv);
    println!("{:?}", adj);
}

const API_URL: &str = "https://api.example.com";

#[test]
fn constan() {
    println!("{:?}", API_URL);
}

#[test]
fn scope() {
    let x = 10;

    {
        //inner scope
        let y = 20;
        println!("{},{}", y, x);
        println!("{}", API_URL)
    }
    // println!("{},{}", x, y);
}

#[test]
fn stack_and_heap() {
    a();
    b();
}

fn a() {
    let x = 10;
    let y = String::from("hello");
    println!("{},{}", x, y);
}

fn b() {
    let x = 10;
    let y = String::from("world");
    println!("{},{}", x, y);
}

#[test]
fn stringS() {
    let s = String::from("hello");
    println!("{}", s);

    let s = " hello pointer ";
    let trim = s.trim();
    println!("{}", s);
    println!("{}", trim);

    let mut firstname = "John";
    println!("{}", firstname);
    firstname = "doe";
    println!("{}", firstname);
}

#[test]
fn string() {
    let mut name = String::from("John");
    name.push_str(" doe");
    println!("{}", name);

    let new = name.replace("doe", "smith");
    println!("{}", new);
}
