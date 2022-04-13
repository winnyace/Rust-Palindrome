#[macro_use]
extern crate text_io;

fn palindrome(mut x: i32) {
    let mut num: i32 = 0; let xc = x;
    while x != 0 {
        num = num*10 +x%10;
        x /= 10;
    }
    if xc == num {
        println!("it is a palindrome");
    } else {
        println!("it isn't a palindrome");
    }
}

fn main() {
    println!("please give me a number. ");
    let input: i32 = read!();
    palindrome(input);
}