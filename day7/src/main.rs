// mutability
// shadowing 같은 이름을 다시 쓰는 것.

fn main() {
    // my_number을 한번 더 사용하고 싶으면 mut 붙임
    // let mut my_number = 10;
    // my_number = 9;

    let my_variable = 10;
    println!("{}", my_variable);
    {
        let my_variable = "My variable";
        println!("{}", my_variable);
    }
    println!("{}", my_variable);

    // let x = 9;
    // let x = double(x);
    // let x = triple(x);
    // println!("{}", x);

}

fn double(input: i32) -> i32 {
    input * 2
}

fn triple(input: i32) -> i32 {
    input * 3
}