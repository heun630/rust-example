/// 문서같이 쓰는 타입 - ducmentation

fn main() {
    // 컴파일러가 전체 다 무시합니다.
    // 이것이 코멘트입니다.
    println!("Hello, world!");

    // let x = 10;
    // warning: unused variable: `x`
    // 안썼을 경우에 이런식으로 오류가 나옴.
    let x/* : i16*/ = 10;

    // 컴파일러한테 일단 무시하라고 지정하는 것
    // _ 언더 스코어로 시작하면 컴파일러가 안바라보게 됨
    let _x = 10;
}
