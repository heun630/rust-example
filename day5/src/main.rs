// () - empty tuple, unit type (void)
// expression-based language

fn empty_tuple() {
}

// fn main () -> ()
// return type이 없는 empty tuple
// main은 return 타입이 있으면 안됨

// Display print {}
// Debug pring {:?}
fn main() {
    let tuple = empty_tuple();
    println!("{:?}", tuple);
}