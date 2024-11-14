fn main() {
   
}

#[test]
 fn imutable(){
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
 fn static_variable(){
    let mut  apple = "apple";
    // APPLE = 2;
    println!("{}", apple);
    apple = "orange";
    println!("{}", apple);
 }

#[test]
fn shadowing(){
    let apple = "apple";
    println!("{}", apple);
    let apple = 10;
    println!("{}", apple);
}


#[test]
fn scalar_data_types(){
    let x = 10;
    let y = 10.0;
    let z = true;
    let a = 'a';
    println!("x: {}, y: {}, z: {}, a: {}", x, y, z, a);
}


#[test]
fn oprations(){
    let x = 10;
    let y = 20;
    let z = x + y;
    let a = x - y;
    let b = x * y;
    let c = x / y;
    let d = x % y;
    println!("z: {}, a: {}, b: {}, c: {}, d: {}", z, a, b, c, d);
}