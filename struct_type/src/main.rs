/*
구조체 (struct)
구조체는 튜플이랑 비슷하다. 하지만 각 구성요소들이 무엇을 의미하는지 명명할 수 있는점이 다르다.
구조체를 정의할 때는 struct 키워드를 이용한다.
중괄호 내에선 구조체의 필드의 이름과 타입을 정한다.
구조체 선언시에 구조체 중괄호에는 세미콜론을 붙이지 않는다.
*/
struct User {
    name: String,
    email: String,
    age: u64,
}

fn main() {
    /*
    구조체를 사용하려면 구조체로부터 인스턴스를 생성해야 한다.
    인스턴스는 아래와 같이 선언한다.
    읽어오는 동작도 다음과 같이 할 수 있다.
    */
    let user1 = User {
        email: String::from("pujuhong"),
        age: 28,
        name: String::from("puju@puju.com"),
    };
    println!("user.name : {}", user1.name);
    println!("user.age : {}", user1.age);
    println!("user.email : {}", user1.email);
    /*
    구조체를 수정 가능하게 하려면 구조체를 mutable하게 선언해야 한다.
    일부만 mutable하게 할 수는 없다.
    */
    let mut user_mutable = User {
        email: String::from("joopark"),
        age: 28,
        name: String::from("puju@puju.com"),
    };
    println!("user_mutable.age : {}", user_mutable.age);
    user_mutable.age = 18;
    println!("user_mutable.age : {}", user_mutable.age);
    /*
    함수의 리턴값이 구조체의 인스턴스가 될 수도 있다.
    난감한 점은 구조체의 필드명과 함수의 지역변수명이나 매개변수명이 같거나 하는 상황이다.
    필드명과 변수명이 같아도 오류가 발생하진 않지만 같은 명칭을 두번 작성하는 것을 피할수 있다.
    */
    let ins1 = gen_user_1(String::from("pujuhong"), 21, String::from("puju@puju.com"));
    let ins2 = gen_user_2(String::from("joopark"), 21, String::from("puju@puju.com"));
    println!("ins1.name : {}", ins1.name);
    println!("ins1.age : {}", ins1.age);
    println!("ins1.email : {}", ins1.email);
    println!("ins2.name : {}", ins2.name);
    println!("ins2.age : {}", ins2.age);
    println!("ins2.email : {}", ins2.email);
    /*
    구조체 갱신법 (struct update syntax)을 이용해 기존 생성된 구조체 인스턴스를 기반으로 다른 값을 가진 구조체를 만들 수 있다.
    변화하지 않은 필드는 .. 구문을 사용해 생략한다.
    */
    let test = gen_user_1(String::from("pujuhong"), 21, String::from("puju@puju.com"));
    println!("before : {}", test.name);
    let test = User {
        name: String::from("joopark"),
        ..test
    };
    println!("after : {}", test.name);
    /*
    튜플이랑 비슷하게 필드의 타입만 정의하고 필드명은 정의하지 않는 튜플 구조체란 것도 있다. (잘 사용하진 않는듯)
    */
    struct Color(u8, u8, u8);

    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);
    println!("black : ({}, {}, {})", black.0, black.1, black.2);
    println!("white : ({}, {}, {})", white.0, white.1, white.2);

    /*
    구조체를 사용하는 프로그램을 만드는 도중 구조체의 구성 요소들을 출력하고 싶을때가 있을것이다.
    println! 매크로에는 디버그라는 출력 포맷을 사용할 수 있는데 이 포맷을 사용하면 구조체의 구성요소들을 출력할 수 있다.
    1. println! 에서 출력할 구조체 인스턴스에 대해 {} 대신 {:?} 혹은 {:#?}을 사용한다.
    2. 해당 구조체 정의 바로 위에 #[derive(Debug)] 어노테이션을 추가한다.
    */
    #[derive(Debug)]
    struct UserTest {
        name: String,
        email: String,
        age: u64,
    }
    let user1_test = UserTest {
        email: String::from("pujuhong"),
        age: 28,
        name: String::from("puju@puju.com"),
    };
    println!("user1_test : {:#?}", user1_test);

    /*
    구조체에 메소드(함수)를 추가할 수 있다.
    메소드는 구조체, 열거형, 트레잇 내부에 정의되며 여기서는 구조체 내부에 정의되는 메소드를 본다.
    구조체의 메소드는 보통 impl(ementation) 블록을 만들고 구조체의 명칭을 적어 정의한다.
    메소드에선 보통 메소드가 소속되어 있는 멤버변수에 접근할 수 있는데 러스트에선 self 파라미터를 이용해 접근하게 한다.
    self를 통해 멤버변수에 접근할 때에도 C++같은 언어처럼 arrow 연산자를 사용할 필요가 없다.
    이는 러스트가 알아서 자동참조와 역참조를 해주기 때문이다.
    self 키워드를 가진 메소드를 호출할 때엔 self 파라피터를 추가할 필요가 없으며
    구조체 내부 값을 사용하지 않으면 self 키워드를 생략할수도 있다. (이를 연관함수라고 한다)
    보통 연관함수는 구조체의 인스턴스를 반환하는 생성자로서 많이 사용한다.
    연관함수를 호출할 때에는 :: 을 사용하고 메소드를 호출할 떄에는 . 을 사용한다.
    */
    struct Rectangle {
        width: u32,
        height: u32
    }
    impl Rectangle {
        fn gen(width: u32,  height: u32) -> Rectangle {
            Rectangle {width, height}
        }
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }
    let rec1 = Rectangle::gen(10, 13);
    println!("rec1 size : {}", rec1.area());
}

fn gen_user_1(name: String, age: u64, email: String) -> User {
    User {
        name: name,
        age: age,
        email: email,
    }
}

fn gen_user_2(name: String, age: u64, email: String) -> User {
    // 필드 초기화 축약법(field init shorthand) 을 이용해 명칭이 동일한 변수명이 들어간다면 필드명을 생략한다.
    User {
        name,
        age,
        email,
    }
}