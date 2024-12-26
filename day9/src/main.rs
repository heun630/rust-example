use std::mem;

fn main() {
    // String -> heap에 있음. Sized type
    // &str -> stack에 있음. Dynamic type
    // &str ref str "string slice"
    let my_name = "David".to_string(); // &str + to_string() => String
    let other_name = String:: from("David");
    let mut my_other_name = "David3".to_string();
    my_other_name.push('!');
    println!("{}", my_other_name);

    // String
    // .capacity
    // .push
    // .push_str
    // .pop
    // with_capacity
    // allocation
    let mut my_name = String::with_capacity(26);
    println!("Length is {} and Capacity is: {}", my_name.len(), my_name.capacity());
    my_name.push_str("David!");
    println!("Length is {} and Capacity is: {}", my_name.len(), my_name.capacity());
    my_name.push_str(" and I live in Seoul");
    println!("Length is {} and Capacity is: {}", my_name.len(), my_name.capacity());

    // Capacity가 26으로 고정되어 있는 상태에서 push가 일어나면 re-allocation의로 인해서 Capacity가 26 -> 52가 된다.(주의)
    my_name.push('a');
    println!("Length is {} and Capacity is: {}", my_name.len(), my_name.capacity());

    let x = 42; // 스택 데이터
    let s = String::from("hello"); // 힙 데이터

    println!("x의 주소: {:p}", &x); // 스택의 주소
    println!("s의 주소 (스택): {:p}", &s); // s 변수 자체의 주소
    println!("s가 가리키는 힙의 주소: {:p}", s.as_ptr()); // 힙 데이터의 주소

    let mut my_string = String::new();
    my_string.push_str("Hello, ");
    my_string.push('R');
    my_string.push('u');
    my_string.push('s');
    my_string.push('t');
    println!("{}", my_string);

    // let my_name_str = "David";
    // my_name_str.push('R');

    let s = "Hello, World!";
    println!("{}", s);

    let full = "Hello, World!";
    let part: &str = &full[0..5]; // "Hello"
    println!("{}", part);

    fn count_chars(s: &str) -> usize {
        s.len()
    }

    let my_string = "Hello, world!";
    let length = count_chars(my_string);
    println!("Length: {}", length);
}
