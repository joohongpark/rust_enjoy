fn main() {
    /*
    러스트도 많은 다른 언어들처럼 힙에 데이터를 저장하는 자료구조를 제공해준다.
    러스트에서는 컬렉션이라고 부르며 보통 벡터, 스트링, 해쉬맵을 많이 사용한다.
    */

    /*
    벡터는 메모리에 원소들이 차례대로 나열된 데이터 구조이다.
    C++ STL의 vector와 유사한 자료구조이며
    러스트의 벡터도 여러 타입의 값을 저장할 수 있기에 제네릭을 이용해 타입을 지정한다.

    새로운 벡터를 만드는 방법은 두가지가 있다.
    첫번째로 Vec::new() 함수를 호출하는 것과
    두번째로 vec! 매크로를 호출하는 방법이 있다.

    매크로를 이용해 벡터를 선언하면 변수에 타입 명시를 할 필요가 없어지며
    초기값들을 가지고 있는 벡터를 만들 수 있다.

    벡터 내에 값을 쓰혀면 변수를 mutable하게 선언하여야 한다.
    push 메소드를 사용하여 마지막 요소에 값을 추가할 수 있다.
    그래서 아래의 vec1은 값을 읽고 쓰기가 가능하며 vec2는 읽기밖에 안된다.

    벡터와 벡터의 요소들은 다른 요소와 유사하게 스코프를 벗어나면 모두 drop된다.
    */
    let mut vec1: Vec<i32> = Vec::new();
    let vec2 = vec![1, 2, 3];

    vec1.push(123);
    vec1.push(456);

    /*
    벡터의 요소를 읽어보자. 요소를 읽는 동작을 뒤에 설명한 이유는 벡터는 보통 인덱스로 읽어오는 경우가 많은데
    인덱스를 잘못 참조하는 경우와 그에 대한 처리를 고려해야 하기 때문이다.

    벡터의 요소는 대괄호로 인덱스를 이용해 벡터 내의 요소에 접근할 수 있다.
    혹은 get 메소드를 이용해 읽어오는 방법이 있다.

    두 가지 방법의 큰 차이가 있는데 인덱스를 통해 값을 가져올 때
    존재하지 않는 인덱스에 접근하면 패닉이 발생한다.
    하지만 get 메소드를 통해 값을 가져오면 값을 직접 주는것이 아닌
    Option 열거형을 반환하기 때문에 패닉을 방지할 수 있다.
    */

    let vec2_idx = vec2[2];
    let vec2_get = vec2.get(2);
    let vec1_get = vec1.get(2);
    println!("vec2[2] : {}", vec2_idx);
    if let Some(n) = vec2_get {
        println!("vec2.get(2) : {}", n);
    }
    if let Some(n) = vec1_get {
        println!("vec1.get(2) : {}", n);
    }

    /*
    소유권과 빌림 규칙은 벡터를 안전하게 사용하게 하는데 영향을 준다.
    같은 스코프 내에서 가변 참조자와 불변 참조자를 가질 수 없는 규칙은 여기에도 적용된다.
    밑 코드에서 println 주석을 지우면 컴파일 에러가 발생하는데
    만약 vec3에 원소를 추가할 때 아예 새로운 공간에 메모리를 새로 할당한다.
    근데 할당에 실패하여 vec3_first가 유효하지 않은 메모리를 가리키게 될 수도 있다.
    소유권가 빌림 규칙이 그런 상황에 빠지지 않도록 돕는 것이다.
    */
    let mut vec3 = vec![1, 2, 3, 4, 5];
    let vec3_first = &vec3[0];
    vec3.push(6);
    //println!("vec3_first : {}", vec3_first);

    /*
    벡터의 요소들에 접근할때에는 보통 for .. in 문을 사용한다.
    for 문에서 벡터를 적용할 필요가 있을 때에는 for 문에도 mut 키워드를 붙여야 한다.
    */
    let vec4 = vec![1, 2, 3, 4, 5];
    for i in &vec4 {
        println!("{}", i);
    }

    /*
    열거형과 벡터를 응용하면 벡터에 여러 가지 자료형을 담게 할 수도 있다.
    */
    enum Mix {
        Int(i32),
        Float(f64),
        Text(String)
    };
    let vec5 = vec![
        Mix::Int(1024),
        Mix::Float(3.14),
        Mix::Text(String::from("헬로월드"))
    ];
    for elem in &vec5 {
        match elem {
            Mix::Int(data) => println!("정수 : {}", data),
            Mix::Float(data) => println!("실수 : {}", data),
            Mix::Text(data) => println!("문자열 : {}", data),
        }
    }

    /*
    러스트에서 스트링은 생각보다 복잡한 개념이다.
    러스트에서 스트링은 크게 두가지 종류가 있다.
    str : 스트링 슬라이스 타입이며 UTF-8 로 저장된 데이터이다. 프로그램의 바이너리에 저장되어 있다.
    String : 러스트의 표준 라이브러리를 통해 제공하는 타입이다. 문자열만으로 이뤄진게 아니며 문자열을 힙 영역에 저장한다.
    */
    let mut empty_str = String::new(); // 비어있는 문자열
    let mut str_one = "hello string".to_string(); // 스트링 리터럴의 메소드를 이용하여 문자열을 만듬
    let str_two = String::from("hello string"); // 스트링의 from 함수를 이용해 문자열을 만듬

    /*
    스트링이 mutable할 경우 스트링과 스트링 슬라이스를 합칠 수도 있다.
    한 글자는 push() 로 합칠 수 있다.
    스트링과 스트링은 + 연산자나 format! 매크로로 합칠 수 있다.
    + 연산자는 사실 함수이며 오른쪽에는 반드시 참조자를 사용하여야 한다.
    왜냐 하면 함수의 인자로 String이 아니라 &str을 받도록 되어 있는데
    &String은 &str로 강제될 수 있다. 이를 역참조 강제(deref coercion) 라고 하며 나중에 다룬다.
    왼쪽에 있는 스트링은 self를 통해 함수 내부로 이동되어 왼쪽에 있는 스트링의 소유권을 반환하기 때문에
    왼쪽에 있는 변수는 더하기 연산을 하고 나면 무효화된다.
    format! 매크로를 사용하면 소유권 걱정 없이 쉽고 편하게 합칠 수 있다.
    */
    empty_str.push_str("hello");
    println!("str : {}", empty_str);
    str_one.push('!');
    println!("str : {}", str_one);
    let str_new = str_one + &str_two;
    println!("str : {}", str_new);
    let str_cat = format!("{}, {}, {}", empty_str, str_new, str_two);
    println!("str : {}", str_cat);

    /*
    스트링의 인덱싱은 다른 언어들에서의 스트링 인덱싱처럼 자유분방하지는 않다.
    러스트에서의 스트링은 Vec<u8>을 감싼 것이라고 한다. 그래서 스트링도 결국 8바이트 단위로 데이터가 저장된다.
    하지만 러스트에선 인덱스를 이용해 문자 하나에 접근하게 하지 않는다.
    이에 대한 근본적인 이유는 러스트에서 문자열을 UTF-8로 저장하며
    영문자를 제외하고 사실상 다른 언어들은 2바이트 이상의 요소로 문자 하나가 구성되기 때문이다.
    러스트에선 이런 이유로 단일 정수로 대괄호를 이용한 스트링에 대한 인덱싱을 허용하지 않는다.
    그래서 스트링을 인덱싱하려면 대괄호와 범위를 기재해야 한다.
    하지만 이 방법은 유효하지 않은 범위 (여기서 유효하지 않은 범위란 인덱싱한 부분이 온전한 문자로 떨어지나를 의미함)가 들어오면 오류가 난다.
    */
    let str_test = String::from("english, 한글, 😈");
    // println!("char : {}", str_test[0]); // `String` cannot be indexed by `{integer}`
    let sub = &str_test[0..1];
    println!("char : {}", sub);

    /*
    그래서 문자열을 인덱싱할 때 사용자의 의도가 명확하게 전달해야 한다.
    바이트 단위로 접근하는 것을 아예 제한하는 것은 아니기 때문이다.
    chars() 메소드는 실제 문자 각각에 대해 쪼갤 수 있도록 해주며
    bytes() 메소드는 바이트 단위로 쪼갤 수 있도록 해준다.
    */
    for c in str_test.chars() { // 이렇게 하면 영문자, 한글, 이모지 각각의 단위로 슬라이싱한다.
        println!("char : {}", c);
    }
    for c in str_test.bytes() { // 이렇게 하면 다른 언어들처럼 1바이트 단위로 슬라이싱한다.
        println!("byte : {}", c);
    }

    /*
    해쉬맵은 키와 값을 쌍으로 가진 자료구조이다.
    키와 값의 매핑은 해쉬 함수를 통해 이루어진다.
    해쉬맵은 std::collections::HashMap 에 정의되어 있으므로
    길게 늘여쓰기 싫으면 네임스페이스를 지정해야 한다.
    원소는 insert 메소드로 집어 넣는다.
    값에 대한 접근은 get 메소드로 한다.
    키에 대한 값이 존재하지 않을수도 있으므로 반환값은 Option으로 감싸져서 온다.
    */
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    if let Some(n) = score {
        println!("score : {}", n);
    }

    /*
    해쉬맵도 벡터와 유사한 방법으로 키-값 쌍에 대한 반복작업을 허용한다.
    */
    for (k, v) in &scores {
        println!("{} : {}", k, v);
    }

    /*
    해쉬맵에 이미 존재하는 키-값 쌍의 값에 값을 갱신하는 방법이 있다.
    1. 기존에 존재하는 값을 그대로 덮어씌운다
    2. 키-값 쌍이 없을 때에만 삽입한다.
       entry 메소드를 호출해 열거형 Entry를 리턴받으며 해당 키가 존재할 경우 키에 대한 값을 리턴받고
       키카 존재하지 않을 경우 or_insert 메소드로 값을 집어넣는다.
    3. 기존 값을 기반으로 값을 덮어씌운다.
       entry 메소드는 키에 대한 값을 리턴해줄 수 있으므로 그 값을 역참조하여 직접 접근한다.
       역참조는 아스탈리스크 연산자를 이용한다.
    */
    // 1번 방법
    scores.insert(String::from("Yellow"), 60);
    // 2번 방법
    scores.entry(String::from("Green")).or_insert(50); // 삽입
    scores.entry(String::from("Blue")).or_insert(50); // 삽입하지 않음
    // 3번 방법
    let val = scores.entry(String::from("Blue")).or_insert(50);
    *val += 10; // 아스탈리스크 연산자를 사용해 
}
