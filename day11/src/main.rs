// const: 대문자로 변수 설정하고
// 어떤 타입인지 무조건 써야함
// attribute로 설정해주면 경고 문구 안뜸
const SCORE: i32= 20; // global scope

// static
// 같은 메모리에 저장됨
// unsafe {}로 하지만 잘 사용안함 왜냐하면 다른 대체재들이 많음
static mut LOW_SCORE: i32 = 0;

// 'static lifetime' 프로그램의 처음부터 끝까지 살 수 있는 것
// lifetime
fn print_score() {
    println!("The score is {}", SCORE);
}


fn main () {
    print_score();

    let my_name = "David"; // &'static str
    unsafe {
        LOW_SCORE = 1;
    }
}