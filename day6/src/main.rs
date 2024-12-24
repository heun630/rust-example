// fn give_number(one: i32, two: i32) -> i32 {
//
// }

fn print_number(one: i32, two: i32) -> i32 {
    // let multiplied = one * two;
    // println!("{}", multiplied)

    let multiplied_by_ten = {
        let first_number = 10;
        first_number * one * two
    };

    multiplied_by_ten
}

fn main () {
    // let my_nm = give_number(9, 8);
    // println!("{}", my_nm);

    let my_number = print_number(9,1);
    println!("{}", my_number);
    // let x = 5;  // 오류: 세미콜론 누락
    // let xx = x + 1;
}