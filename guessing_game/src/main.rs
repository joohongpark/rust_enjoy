use std::io; // 표준 라이브러리의 입츌력 라이브러리를 가져옴. C++에서 using namespase와 비슷한듯

fn main() {
    println!("숫자를 입력하세여"); // 아직 왜 그런진 모르겠지만 println 함수가 아니라 매크로를 가져와야 한다 함.
    println!("뭘 입력했는지 예상해보겠습니다");
    let mut guess = String::new(); // 러스트에서 변수는 기존적으로 불변속성인데 mut(able) 키워드를 붙이면 가변변수가 됨
    /*
    String은 표준 라이브러리에서 제공하는 확장 가능한 UTF-8 문자열 타입
    new() 함수는 String의 정적 메서드이다. (C++같은 언어에서 클래스의 정적 멤버함수를 호출하는것과 동일하다.)
    하지만 러스트에선 정적 메서드가 아니라 연관 함수 (Associated function) 라고 부르는 듯 하다.
    아무튼 new 함수는 비어있는 String 타입을 생성하여 guess에 대입한다.
    */
    io::stdin().read_line(&mut guess).expect("라인 가져오기 실패");
    /*
    io의 연관함수인 stdin() 함수를 호출한다.
    참고로 use 키워드를 사용하지 않는다면 std::io::stdin() 이렇게 써야한다.

    std::io::stdin()의 리턴값은 터미널 표준 입력의 핸들러이며 이 핸들러는 read_line 이라는 메소드를 가지고 있다.
    read_line 메소드는 사용자가 입력한 한 줄을 인자에 넘긴다.
    read_line의 인자를 살펴보자. 3가지 파트로 이루어져 있다.
        & : 참조자라고 부른다. 참조자는 복잡한 개념이기에 추후에 알아본다.
        mut : read_line의 동작 방식은 사용자가 문자열을 입력하는 것을 실시간으로 변수에 삽입한다고 한다. (한번에 한 줄을 삽입하는게) 아니라
              그래서 가변 속성을 가지고 있어야 하기 때문에 mut 키워드를 붙여야 한다고 함.
        guess : 값을 저장할 문자열 변수
    
    read_line도 리턴값이 존재하며 이 리턴값은 expect라는 메소드를 가지고 있다.
    값의 타입은 io:Result이며 이 타입은 열거형이다.
    멤버 변수로는 Ok와 Err가 있다고 한다.
    expect 메소드는 멤버변수가 Ok라면 별 동작을 하지 않으며 Err일때에는 인수로 입력된 메시지를 출력한다. (프로그램도 중지되는듯)
    컴파일러는 expect 메소드를 붙이지 않으면 경고를 발생시킨다. (컴파일은 된다.)
    */
    println!("님이 입력한 숫자는 {}입니다.", guess);
    /*
    placeholder라는게 있는데 {} 변경자를 사용하여 값이 표시되는 위치를 나타낸다. C의 printf에서 % 붙여서 값 표기하는거랑 비슷하다.
    */
}
