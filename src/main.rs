fn main() {
    let tup = (500, 6.4, 1);

    let (x,y,z) = tup;

    println!("The value of y is {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hund = x.0;
    let six_point_four = x.1;
    let one = x.2;

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    let index = 10;

    // let element = a[index];
    // println!("The value of element is: {}", element);



}


/*

Data Type

1. 스칼라 - 하나의 값으로 표현되는 타입

    정수형
        i는 -, + u는 양수만
        isize, usize = 프로그램 동작하는 환경에 따라 64bit, 32bit

    부동소수점
        기본타입 f64

    boolean

    문자
        작은 따옴표로 표현함

2. 컴파운드

    튜플

    배열
        Rust의 배열은 고정된 길이를 가짐 -> 커지거나, 작아질 수 없음


Rust는 컴파일 타임에 무조건 타입이 정해져있어야 함


*/