fn main() {
    println!("Understanding Ownerships in Rust!");
    /*
    The rules of Ownerships:
    1. Each value in Rust has an owner.
    2. There can only be one owner at a time.
    3. When the owner goes out of scope, the value will be dropped.
    */

    {
        let s = "Hello";
        println!("{s}");
    }

    // println!("{s}"); the s variable will not work here cuz it's out of scope
}
