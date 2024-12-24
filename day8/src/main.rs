fn main() {
    /*
    let my_number = 15; // This is an i32
    let single_reference = &my_number; //  reference to my_number
    let double_reference = &single_reference; // This is a &&i32
    let five_references = &&&&&my_number; // This is a &&&&&i32
     */
    println!("This");
    println!("This\nis\na\nbunch\nof\nlines");
    println!("This\\nis\\na\\nbunch\\nof\\nlines");
    println!(r#"c:\thisdrive\new_drive"#); // raw text
    println!(
"Let me tell you
어떤 이야기를
봅시다.");

    // pointer 위치 프린팅
    let my_variable = 9000;
    let pointer = &9000;
    println!("{:b}", my_variable);
    println!("{:p}", pointer);
    println!("{:x}", my_variable);

    let title ="TODAY'S NEWS";
    println!("{:-^30}", title);
    let bar = "|";
    println!("{: <15}{: >15}", bar, bar);
    let a = "SEOUL";
    let b = "TOKYO";
    println!("{city1:-<15}{city2:->15}", city1 = a, city2 = b)
}