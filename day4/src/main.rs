fn main() {
    // 중간에 _를 넣어도 compiler가 보지 못함
    // let my_number = 9_u8;
    // let other_number = 1000_000_00_u64;

    // .만 붙여도 float이 된다.
    // 타입을 맞춰줄 때, as를 붙여서 맞춰줄 수 있다.
    let my_number = 9.; // f64
    let other_number = 9; // i32
    println!("{}", my_number as i32 + other_number);
    println!("{}", my_number + other_number as f64);

    // macro = function that writes code
    println!("Hello, world!");

    let my_name = "David";
    let my_age = 42;
    println!("My name is {} and my age is {}", my_name, give_age());
    // println!("My name is {} and my age is {}", my_name, my_age);
    println!("My name is {my_name} and my age is {}", my_age);

    let my_city = "Seoul";
    let year = 2002;
    let population = 9_987_987;


    println!("The city of {} in {} had a population of {}", my_city, year, population);

    // string interpolation
    println!("The city of {my_city} in {year} had a population of {population}");

    // 이거는 안되는 것
    // println!("The city of {my_city} in {year + 2} had a population of {population}");

    println!(
        "The city of {0} in {1} had a population of {2}. I love {0}!",
        my_city,
        year,
        population
    );

    println!("The city of {} in {} had a population of {}", my_city, year + 2, population);
}

fn give_age() -> i32 {
    // 다른 언어에서는 return 쓰는데
    // rust에서는 return 꼭 쓸 필요가 없음 (쓰기도 함)
    42
}