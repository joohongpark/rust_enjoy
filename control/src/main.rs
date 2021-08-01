fn main() {
    /*
    러스트의 제어문
    프로그래밍 언어에서 가장 흔하게 사용되는 if 제어문의 특징은 if 표현식에 소괄호를 작성하지 않는 것이다. (작성하면 컴파일 워닝이 뜬다.)
    다른 언어와 비슷하게 if 표현식이 boolean이어야 하며 true이어야 실행된다.
    if 표현식의 조건은 갈래 (arms)라고 부른다.
    너무 많은 if - else if 문은 가독성도 떨어지므로 추후에 match라는 분기 생성자를 사용하게 될 것이다.
    */
    let number = 10;
    if number % 4 == 0 {
        println!("number는 4의 배수입니다.");
    } else if number % 3 == 0 {
        println!("number는 3의 배수입니다.");
    } else if number % 2 == 0 {
        println!("number는 2의 배수입니다.");
    } else {
        println!("number는 4, 3, 2로 나눌 수 없습니다.");
    }
    /*
    let 구문에서 if문을 사용할 수 있다. 이게 가능한 이유는 if가 구문이 아닌 표현식이기 때문이다.
    */
    let is_even = if number % 2 == 0 {
        true
    } else {
        false
    };
    if is_even {
        println!("{} is even", number);
    } else {
        println!("{} is odd", number);
    }
    /*
    몇 가지 반복문이 있다.
    loop, while, for가 있다.
    loop는 C나 C++같은 언어에서 while (1) 과 동일하게 동작한다.
    break 키워드를 사용해서 루프를 벗어나지 않는 이상 무한히 반복한다.
    while문도 제공한다. while문은 조건문이 true일 때에만 계속 반복한다.
    for문도 제공한다. for문은 for ~ in 과 같은 형식으로 사용하며 in 뒤에 오는 콜렉션의 원소 개수만큼 순회한다.
    이는 배열의 끝을 넘거나 인덱스를 잘못 참조하거나 하는 염려를 없앨 수 있는 방법론이다.
    혹은 n번 어떤 동작을 반복할 때에도 for문을 사용할 수 있으며 이럴때에는 보통 Range 라이브러리를 사용한다.
    */
    let mut loop_cnt = 5;
    loop {
        if loop_cnt == 0 {
            break;
        } else {
            loop_cnt = loop_cnt - 1;
            println!("loop_cnt : {}", loop_cnt);
        }
    }
    while loop_cnt != 5 {
        loop_cnt = loop_cnt + 1;
        println!("loop_cnt : {}", loop_cnt);
    }
    let arr_ten = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for elem in arr_ten.iter() {
        println!("element -> {}", elem);
    }
    for i in (1..10).rev() { // range 의 rev 메소드를 사용
        println!("element -> {}", i);
    }
}
