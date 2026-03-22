fn main() {
    let s1 = String::from("divyansh");

    let s2 = &s1;
    let s3 = &s1;
    let s4 = &s1;

    println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);
    println!("{}", s4);
    
}
