/*
열거형이 구조체보다 적절한 상황이 있다. 예를 들어 아이피 버전을 나타낼 때 IPv4와 IPv6을 동시에 가지고 있을 수 없다.
열거형을 이를 나타낼 때 아주 적절하다. V4와 V6 둘 다 가질 수 있지만 동시에 가지진 못하기 때문이다.
*/
enum IpAddrKind {
    IPv4,
    IPv6,
}

/*
러스트의 열거형은 단순히 값을 나열하는것 외에 더 많은 기능이 있다. 열거형 변수에 데이터를 직접 삽입할 수 있다.
이렇게 사용하면 다른 언어에서 열거형과 구조체를 섞어서 사용할 때 구조체를 사용할 필요가 더 없어진다.
열거형 내부의 열거형 변수는 각각 다른 타입의 값을 가지고 있을 수 있다.
예를 들면 밑의 Message 열거형은
 - Quit        : 연관 데이터 없음
 - Move        : 익명 구조체 포함
 - Write       : 문자열 포함
 - ChangeColor : 부호있는 32비트 정수
*/
enum IpAddr {
    IPv4(u8, u8, u8, u8),
    IPv6(String),
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

/*
Null이 존재하는 언어에서 Null은 여러 문제를 유발시킨다.
보통 Null이 유발하는 문제는 Null 값을 Null이 아닌것으로 가정하고 접근하는 데서 에러가 난다.
(자바의 NullPointException과 같이 널포인터로 인한 오류가 대표적이다.)
이런 문제를 원천적으로 방지하기 위해 요즘 앵간한 언어에 존재하는 Null이 러스트에는 없다.

러스트에는 값이 존재하는지, 존재하지 않는지의 여부를 Null 대신 다른 방법으로 알려준다.
Option이라는 표준 라이브러리에서 정의된 열거형 타입이 있다.
이 타입은 값이 있을수도 있거나 없을수도 있는 상황을 나타낼 수 있다.

실제 러스트 표준 라이브러리에 구현된 형태는 다음과 같다. 추후에 다룰 것이지만 <T> 는 템플릿이다.

enum Option<T> {
    Some(T),
    None,
}

Option 열거형은 러스트에서 일반적으로 사용되기 때문에? 명시적으로 가져오거나 네임스페이스를 붙이지 않아도 사용 가능하다.
let some_number = Some(5);
let some_string = Some("a string");
let absent_number: Option<i32> = None;
이렇게 쓸 수 있다.

일반적인 언어에서 Null을 이용해 값이 있을지도 없을지도 모르는 상황은 Option 타입을 이용해서 처리하며
이는 의도적으로 설계된 부분이다. 이는 (러스트에는 Null이 없지만) Null의 남발을 막는데도 도움이 되며
안정성에도 도움이 된다.
*/

/*
러스트의 열거형은 열거형에 메소드를 정의할수도 있다.
이렇게 정의할 수 있다는것만 알아두자.
impl Message {
    fn log(&self) {
        println!("{}", self);
    }
}
*/

fn main() {
    /*
    열거형을 선언하는 법을 알게 되었으니 본격적으로 사용해보자.
    위에서 선언한 IP 주소타입에 따라 다른 동작을 하는 코드를 만들어보자.

    러스트에는 match라는 흐름 제어 연산자가 존재한다.
    match는 입력된 값이 어느 패턴에 매칭되는지 판단하여 특정 코드를 수행하게 해준다.
    패턴은 리터럴이나 변수명이나 와일드카드 등이 될 수 있다.

    밑의 예제에서 match 뒤 표현식은 iptype가 되고 match 내의 것들은 match의 갈래(arms)라고 부른다.
    갈래는 패턴과 코드로 이루어지며 그 두개는 => 연산자로 이어진다.
    코드는 보통 한줄이라면 중괄호를 생략하며 코드가 길다면 중괄호로 묶는다.
    match는 C나 C++의 switch처럼 위부터 아래 순서로 순차적으로 패턴과 표현식을 비교한다.

    match는 비교뿐만 아니라 비교된 값을 그 갈래로 바인딩할수도 있다.
    위에서 열거형 변수에 데이터를 삽입했는데 이 데이터를 바인딩시켜서 match 내에서 사용할 수 있다.

    또 match는 모든 케이스를 다 다루도록 명시되어 있어야 한다.
    컴파일러가 검사하며 케이스를 빠트렸을 경우 컴파일러는 오류를 발생시킨다.
    C/C++의 case문에서의 default같은 키워드가 존재하므로 다룰 필요 없는 케이스는 별도로 처리해야 한다.
    _ 변경자 (placeholder)를 사용하면 어떤 값과도 매칭된다.

    match는 C나 C++의 switch보다 다양한 기능들을 지원한다.
    https://doc.rust-lang.org/rust-by-example/flow_control/match.html
    */
    let iptype: IpAddrKind = IpAddrKind::IPv4;
    match iptype {
        IpAddrKind::IPv4 => println!("it is IPv4!"),
        IpAddrKind::IPv6 => println!("it is IPv6!"),
    };
    let ip4: IpAddr = IpAddr::IPv4(192, 168, 0, 1);
    match ip4 {
        IpAddr::IPv4(a, b, c, d) => println!("ip : {}.{}.{}.{}", a, b, c, d),
        IpAddr::IPv6(str) => println!("ip : {}", str),
    };
    let t = Message::Write(String::from("test"));
    if let Message::Write(str) = t {
        println!("str : {}", str);
    }

    /*
    위의 match는 검사할 필요가 없는것도 검사해야 하기 때문에 특정 상황에선 코드가 길어지기만 할 수 있다.
    if let 문법은 if와 let이 조합된 형태로 이 패턴을 이용하면 하나의 조건만 검사할 수 있다.
    if let은 = 로 구분된 패턴과 표현식을 입력받는다.
    */
    let num = Some(10);
    let num_none = None;
    let num_plus = int_plus(num);
    let num_none_plus = int_plus(num_none);
    if let Some(n) = num_plus {
        println!("{} is number!", n);
    }
    if let None = num_none_plus {
        println!("not number!");
    }
}

fn int_plus(num: Option<i32>) -> Option<i32> {
    // match와 Option 열거형 타입을 이용해 안전하게 인수에 숫자 1을 더하는 함수
    match num {
        None => None,
        Some(num) => Some(num + 1)
    }
}