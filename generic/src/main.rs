fn main() {
    /*
    러스트도 제네릭 프로그래밍을 지원한다.
    러스트의 제네릭 프로그래밍은 C++처럼 컴파일 타임에 동일한 형태의 제네릭 코드들을 사용된 타입만큼 찍어낸다.
    이를 단형성화(monomorphization) 라고 한다고 한다.
    그래서 런타임에 성능 하락이 없다.
    구조체, 열거형의 정의나 메소드의 정의에 대해서도 제네릭 데이터 타입을 사용할 수 있다.
    */
    let test_int = Test{data: 123};
    let test_str = Test{data: "str"};
    let get_int = test_int.get();
    let get_str = test_str.get();
    println!("{}", get_int);
    println!("{}", get_str);

    /*
    트레잇 (trait)이란 러스트 컴파일러에게 특정 타입이 가지고 있고 다른 타입들과 함께 공유할 수 있는 기능을 말해주는 역할을 한다.
    그래서 제네릭 타입 파라미터를 쓸 때 컴파일 타임에 이 제네릭 타입이 어떤 트레잇을 구현한 타입이어야 함을 명시해준다.
    그렇지 않으면 타입에 없는 기능이 들어오게 될 때 문제가 되기 때문이다.

    C++의 STL에 비슷한 개념이자 비슷한 용어인 트레잇이 있다. 하지만 러스트의 트레잇은 인터페이스와도 유사하다.

    트레잇을 정의한다라는 말은 트레잇을 구현(특정 타입에 대해 트레잇을 적용)할 때 트레잇 내에 있는 함수 시그니쳐가 반드시 존재하도록 강제하는 것이다.
    마치 자바의 인터페이스와 유사하다.
    
    밑에 A (toString 트레잇 정의)를 보자.
    */
    let p = Pos{x: 10, y: 20};
    let m = Msg{data: String::from("hello~")};
    println!("p.string() : {}", p.string());
    println!("m.string() : {}", m.string());
    m.print();

    /*
    트레잇과 제네릭을 한번에 설명하는 이유가 있다. 제네릭 타입으로 인수를 받으면 그 인수로 무엇인가를 하는 것이 문제이다.
    예를 들어 밑에 구현한 Test는 제너릭 타입의 변수를 가진다. 만약 Test에 대해 출력하는 메소드를 만든다 생각해보자.
    T에는 프리미티브 타입이 올수도 있고 객체가 올수도 있다. 그래서 무작정 println 매크로로 출력한다면 컴파일러가 거부한다.
    그래서 제네릭으로 인수를 받을 때 내부에서 하고자 하는 동작이 정의되었는지 확인을 해야 하는데
    이를 트레잇을 이용해서 확인하는 것이다.

    예를 들어 list_print라는 함수는 리스트 내 인수를 모두 출력해주는 함수이다.
    하지만 트레잇을 고려하지 않으면 컴파일러가 거부한다. 출력 가능한 타입이 올 수도 있기 때문이다.
    그래서 타입에 대해 사용하고자 하는 동작에 대해 트레잇으로 제한을 해야 한다.

    밑 Test 구조체와 다르게 list_print는 자료형 T에 대해 무엇인가를 하기 때문에 트레잇으로 제한(바운드)을 해야 하며
    저 함수에서는 단순히 출력하는 동작밖에 하지 않는다.
    따라서 T: std::fmt::Display 와 같이 트레잇이 구현되었는지 확인하는 구문이 들어가야 한다.

    그리고 for문에서 리스트의 원소를 하나씩 뽑아 스택에 잠시 저장하는 동작을 하는데
    이때 원소에 대해 복사 연산자가 동작하게 된다.
    원소가 복사 연산자를 지원하지 않을 수도 있기 때문에 Copy 트레잇 제한도 추가해야 한다.

    함수 내에서 자료형 T에 대해 여러 작업들을 하다면 + 연산자로 트레잇 제한을 더할 수도 있다.

    트레잇 제한이 늘어나거나 길어질수록 가독성이 떨어지기 때문에 함수를 다음과 같이 작성할 수도 있다.

    [이전]
    fn list_print<T: std::fmt::Display>(list: &[T]) {

    [이후]
    fn list_print<T>(list: &[T])
        where T: std::fmt::Display {
    */
    let l = vec![1, 2, 3, 4, 5];
    list_print(&l);
}

fn list_print<T: std::fmt::Display + Copy>(list: &[T]) {
    for &i in list.iter() {
        println!("{}", i)
    }
}

struct Test<T> {
    data: T
}

impl<T> Test<T> {
    fn get(&self) -> &T {
        &self.data
    }
}

/*
A. toString이라는 트레잇을 정의한다고 생각해 보자.
대상 타입에 대한 정보를 문자열로 출력하고자 하게 하는 트레잇을 정의한다고 가정하는 것이다.
이 트레잇은 string이라는 함수 시그니쳐만을 가진다.

타입 하나하나마다 일일히 정의하는 것이 좋지만
타입에 대한 기본 동작을 정해놓고 필요한 경우 동작을 정의하고자 하는 메소드는 구현 블록까지 정의한다.
*/
pub trait toString {
    fn string(&self) -> String;
    fn print(&self) {
        println!("print 메소드의 기본 동작임")
    }
}
/*
좌표를 나타내는 구조체 Pos가 있다. 또 단순 메시지를 가지고 있는 구조체 Msg가 있다.
*/
pub struct Pos {
    pub x: i32,
    pub y: i32
}

pub struct Msg {
    pub data: String
}
/*
Pos와 Msg에 대해 본인이 가지고 있는 데이터를 출력할 수 있는 string이라는 공통 메소드를 만들 때
기존에 정의한 트레잇을 적용하며 그 구조체에 맞게 구현할 수 있다.
만약 트레잇에 정의된 기본 동작을 따르고 싶을 경우 빈 impl 블록만 남겨둔다.
*/
impl toString for Pos {
    fn string(&self) -> String {
        format!("Pos(x : {}, y : {})", self.x, self.y)
    }
}
impl toString for Msg {
    fn string(&self) -> String {
        format!("Msg(data : {}", self.data)
    }
}