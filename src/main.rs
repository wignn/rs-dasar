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
