use std::mem::size_of;

fn main() {
    // char vs string
    // char은 4bytes

    println!("Size of a char: {} bytes", size_of::<char>()); // 4 bytes

    // .len() = length
    // rust에서는 length는 byte를 얘기함
    println!("Size of string containing 'a': {}", "a".len()); // .len() gives the size of the string in bytes
    println!("Size of string containing 'ß': {}", "ßßßßß".len());
    println!("Size of string containing '国': {}", "国".len());
    println!("Size of string containing '𓅱': {}", "𓅱".len());

    // 글자 수를 알고 싶을 때?
    // .chars().count()
    let slice = "Hello!";
    println!("Slice is {} bytes and also {} characters.", slice.len(), slice.chars().count());

    let slice2 = "안녕!";
    println!("Slice is {} bytes and also {} characters.", slice2.len(), slice2.chars().count());
}
