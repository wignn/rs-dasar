
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
fn test_unit() {
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
fn strings() {
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

#[test]
fn ownership() {
    let a = 10;
    {
        let b = 20;
        println!("{}", b);
    } // setelah scope ini b akan dihapus dari memory karna sudah tidak terpakai
    println!("{}", a);
} // setelah scope ini a akan dihapus dari memory karna sudah tidak terpakai
#[test]
fn data_copy() {
    let a = 10;
    let b = a;
    println!("a: {}, b: {}", a, b);
}

#[test]
fn ownership_movement() {
    let a = String::from("hello");
    let b = a;
    println!("{}", b);
    // println!("{}", a);
}

#[test]
fn ownership_clone() {
    let a = String::from("hello");
    let b = a.clone();
    println!("a: {}, b: {}", a, b);
}

#[test]
fn if_statement() {
    let a = 10;
    let result: &str;
    if a > 10 {
        result = "a lebih dari 10";
    } else if a < 10 {
        result = "a kurang dari 10";
    } else {
        result = "a sama dengan 10";
    }
    println!("{}", result);

    let result = if a > 10 {
        "a lebih dari 10"
    } else if a < 10 {
        "a kurang dari 10"
    } else {
        "a sama dengan 10"
    };
    println!("{}", result);
}

#[test]
fn loop_statment() {
    let mut init = 0;

    loop {
        init += 1;
        println!("iterasi ke {}", init);
        if init > 10 {
            break;
        } else {
            continue;
        }
    }

    let mut init = 0;
    let result = loop {
        init += 1;
        if init > 10 {
            break init;
        } else if init % 2 == 0 {
            println!("genap {}", init);
            continue;
        }
        {}
    };
    println!("{}", result);
}

#[test]
fn while_loop() {
    let array = ["a", "b", "c", "d"];
    let mut i = 0;
    while i < array.len() {
        println!("{}", array[i]);
        i += 1;
    }
}

#[test]
fn for_loop() {
    let array = ["a", "b", "c", "d"];

    for value in array {
        println!("{}", value);
    }
}

#[test]
fn for_loop_with_range_exclusif() {
    let array = ["a", "b", "c", "d"];
    // let range = 0..array.len();

    for i in 0..4 {
        println!("{}", array[i]);
    }
}

#[test]
fn for_loop_with_range_inclusif() {
    let array = ["a", "b", "c", "d"];
    // let range = 0..array.len();

    for i in 0..=3 {
        println!("{}", array[i]);
    }
}

fn say_hello(text: &str) {
    println!("{}", text);
}

#[test]
fn function() {
    say_hello("hello");
}

#[test]
fn function_with_return() {
    calculate(1, 20);
}

fn calculate(a: i32, b: i32) -> i32 {
    for i in a..b {
        if i % 2 == 0 {
            println!("{}", i);
        } else {
            continue;
        }
    }
    return a + b;
}

#[test]
fn function_with_owneship() {
    let a = String::from("hello");
    let b = String::from("world");
    let result = combine(&a, &b);
    // println!("{}", a);
    println!("{}", result);
}

fn combine(a: &String, b: &String) -> String {
    let result = format!("{} {}", a, b);
    return result;
}

#[test]
fn pointer_reference() {
    let a = String::from("hello");
    let b = String::from("world");
    let result = combine(&a, &b);
    println!("{}", a);
    println!("{}", b);
    // println!("{}", a);
    println!("{}", result);
}

#[test]
fn borrowing_test() {
    let mut a = String::from("hello ");
    borrowing(&mut a);
    println!("ini adalah mut pointer: {}", a);
}

fn borrowing(a: &mut String){
    //secara default borrowing adalah immutable
     a.push_str("world");
}


#[test]
fn slice_test() {
    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    //selice is a reference to a part of a string
    let slice_full = &array[..];
    println!("slice 1: {:?}", slice_full);

    let slice_to_end = &array[5..];
    println!("slice 2: {:?}", slice_to_end);

    let slice_to_start = &array[..5];
    println!("slice 3: {:?}", slice_to_start);
}


#[test]
fn string_slice (){
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);
}


struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[test]
fn struct_test(){
let mut users = [
    User {
        username: String::from("example1"),
        email: String::from("example1@example.com"),
        sign_in_count: 1,
        active: true,
    },
    User {
        username: String::from("example2"),
        email: String::from("example2@example.com"),
        sign_in_count: 2,
        active: false,
    },
];



for user in &users {
    parameter_struct(&user);
}
}


fn parameter_struct(users: &User){
    println!("{}, {}, {}, {}", users.username, users.email, users.sign_in_count, users.active);
}


struct  GeoPoint(f64, f64);

#[test]
fn struct_tupe(){
    let point = GeoPoint(10.0, 20.0);
    println!("{}, {}", point.0, point.1);
}


struct nothing;
#[test]
fn struct_without_field(){
    let _black = nothing;

}