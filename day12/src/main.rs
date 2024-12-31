fn return_string() -> &'static String {
    let country = String::from("대한민국");
    &country
}

fn main () {
    let my_country = return_string();
}