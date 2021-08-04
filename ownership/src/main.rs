fn main() {
    /*
    소유권 (Ownership)

    러스트의 핵심 기능은 소유권이다.
    C나 C++같은 언어는 데이터가 스택이나 힙에 할당되는 것을 신경써서 코딩해야 한다.
    자바같은 경우는 GC가 존재해 미사용 메모리 공간 (힙) 을 알아서 반환해 준다. 하지만 GC때문에 느려진다.
    C나 C++가 어려운 이유 중 하나는 힙에 할당되는 메모리 공간을 신경써서 코딩해야 한다는 점이며
    러스트에서는 소유권이라는 개념으로 코딩을 할 때 스택 혹은 힙에 할당되는 것을 크게 신경쓸 필요가 없도록 해준다.

    소유권 규칙은 크게 세가지가 있다.
        1. 러스트의 각각의 값은 해당값의 오너라고 부르는 변수를 가지고 있다.
        2. 한번에 하나의 오너만 존재할 수 있다.
        3. 오너가 스코프 밖으로 벗어나면 그 값은 버려진다.
    */

    /*
    변수의 스코프에 대해 알아보자.
    변수는 스코프 내에서만 유효하다.
    밑에 a와 s라는 변수가 있는데 a 변수는 23줄부터 24줄까지만 유효하며 s 변수는 26줄부터 메인함수가 끝날때까지 유효하다.
    둘 다 각각의 스코프 내에서만 유효한 것이다.
    */
    {
        let a: u32 = 1234;
        println!("a : {}", a);
    }
    let b = 4321;
    println!("b : {}", b);

    /*
    단순한 데이터 타입(숫자, 문자 등)으로 변수를 선언하면 보통 스택 영역에 저장되고 함수콜이 끝나면 pop된다.
    아마 위의 u32 자료형이 main 함수가 콜될때 스택 영역에 push 될 것이다.
    하지만 복잡한 데이터 타입은 보통 힙에 저장되는 편이며 이런 부분을 잘 신경써서 보자.

    아래의 String 타입과 스트링 리터럴은 조금 다르다.
    아래의 스트링 타입은 문자열 추가나 삭제가 가능하다. 하지만 스트링 리터럴은 불가하다.
    스트링 리터럴은 컴파일 타임에 결정되지만 스트링 타입은 런타임에 결정되며 값이 변경될 수도 있어야 한다.

    아래의 String::from 을 실행하면 런타임에 메모리를 요청하는 (C++에서 new std::stirng("hello~") 하는것과 비슷하게) 동작을 하며
    할당된 메모리의 사용이 끝나면 할당을 해제해야 한다.
    C나 C++같은 경우 free나 delete로 할당을 해제해야 하며 자바같은 언어는 GC가 동작해 런타임에 알아서 찾아서 할당을 해제해준다.
    C나 C++같은 경우는 사용자가 필요한 순간에 메모리를 할당하고 적정 순간에 해제하지만 오류가 발생될 여지가 크다.
    자바같은 경우는 사용자가 해제할 필요 없어 안전한 편이지만 GC가 구동되므로 비효율적이다.

    러스트가 C/C++이나 자바와 차별화 되는점은 이 문제를 다르게 다루는 것이다.
    러스트는 힙 영역에 할당한 변수가 스코프를 벗어나는 순간 자동으로 반납시킨다.
    예를 들어 밑의 s_heap은 53번째 줄에서 힙 영역에 할당되지만 사용자가 할당 해제할 필요가 없으며
    main 함수가 종료되 s_heap이 스코프를 벗어나면 알아서 반납이 된다.
    (러스트에서는 drop이라는 함수로 반환시키는데 스코프를 벗어나면 러스트가 알아서 해준다.)

    아래의 s_heap은 엄밀히 말하면 String의 ptr 요소만 힙에 저장되며 len, capacity 요소는 스택에 저장된다.
    */
    let mut s_heap = String::from("hello~");
    s_heap.push_str(" World~");
    println!("s_heap : {}", s_heap);

    let s_bin = "hello";
    println!("s : {}", s_bin);

    /*
    아래는 변수와 데이터가 상호 작용하는 예이며 이동의 예제를 보여준다.
    프리미티브 타입의 변수 x와 y는 스택에 5라는 값이 push되는 동작을 한다. (x와 y 변수는 독립적이지만 같은 값을 가지고 있다.)

    하지만 힙에 할당된 s_x와 그 값을 복제하는 s_y는 조금 다르다.
    71번째 줄에서 생성한 s_x는 72번째 줄에서 복사가 된 이후 73번째 줄에선 무효해진다.
    내부적으로는 73번째 줄에서 "얕은 복사 - len, capacity 값만 복사하고 ptr은 포인터 값만 복사"를 하고 s_x 변수를 무효화시킨다.
    그래서 C++에서 비슷하게 클래스 만들고 얕은 복사를 하게 될때 double free 되는것과 같은 일이 일어날 수 없게 하는 것이다.

    러스트에서 보통 데이터에 대해 얕은 복사만 수행한다. (객체를 복제할 때 계속 깊은 복사를 하면 느려지기 때문에 그런듯)
    사용자가 원한다면 깊은 복사를 하게 할 수 있긴 하다. (객체의 clone() 메소드를 호출하면 된다.)
    하지만 보통 깊은 복사는 보다 큰 비용을 소모하기 때문에 명시적으로 그런 동작이 발생하는 것을 알려주는 역할도 한다.

    러스트는 정수형같은 프리미티브 타입은 (컴파일 타임에 크기가 정해지는) 스택에 저장하며 이에 대한 특별한 어노테이션을 달 수 있다.
    (지금 알 필요는 없고 추후에..)
    */
    let x = 5;
    let y = x;
    println!("x : {}, y : {}", x, y);

    let s_x = String::from("hello~");
    let s_y = s_x;
    println!("s_y : {}", s_y);
    //println!("s_x : {}, s_y : {}", s_x, s_y);

    /*
    이제 함수의 인자로 변수를 넘기는 경우를 생각해보자.
    함수의 인자로 변수를 넘기는 것도 위의 대입과 비슷하게 다른 공간으로 복사가 된다.
    자연스럽게 복사가 되는 프리미티브 한 변수는 함수의 인수로 전달될 때 그 함수 내부로 복사가 된다.
    하지만 위처럼 힙 영역에 할당되는 변수는 소유권이 그 함수로 넘어가버리기 때문에 함수를 호출하여 인수를 인자로 넣어줄 때
    함수를 빠져나오고 나서는 그 변수는 사용할 수 없게 된다.
    */
    let v_int = 1234;
    let v_str = String::from("string");
    get_integer(v_int);
    get_string(v_str);
    println!("v_int : {}", v_int);
    //println!("v_str : {}", v_str);

    /*
    함수에서 리턴되는 값도 힙 영역에 할당되는 부분이 있다면 소유권이 함수 밖으로 빠져나간다.
    변수의 소유권은 항상 패턴이 같다.
    어떤 값을 다른 변수에 대입하면 값을 이동된다.
    만약 힙에 데이터를 가진 변수가 스코프 밖으로 벗어나면
    해당 값은 데이터가 다른 변수에 의해 소유되도록 이동하지 않는 한 drop에 의해 알아서 제거된다.
    그래서 아래처럼 strlen 함수를 만든다 할 때 문자열을 그대로 집어넣으면 이 함수 호출 이후로 문자열을 사용하지 못한다.
    strlen_fix 함수처럼 대입한 변수명과 동일한 명칭으로 쉐도잉을 하는 방법으로 해결할 수 있긴 하지만 이런 방법은 지저분하다.
    */
    let v_str_1 = String::from("string");
    let v_str_2 = String::from("string");

    let len1 = strlen_bad(v_str_1);
    let (len2, v_str_2) = strlen_fix(v_str_2);
    //println!("v_str_1 : {}", v_str_1);
    println!("v_str_2 : {}", v_str_2);
    println!("v_str_1.len() : {}", len1);
    println!("v_str_2.len() : {}", len2);

    /*
    위와 같은 불편함을 해소하기 위해 러스트에는 참조자(레퍼런스)라는 개념이 있다.
    앤드 기호(ampersand) 연산자를 이용해 함수를 선언할때(함수 시그니쳐)랑 함수에 대입할 때(C,C++에서 변수 포인터 가져오는거마냥) 참조자임을 명시하며
    참조자를 사용하면 호출된 함수에서 소유권을 가져가지 않고 변수에 접근할 수 있도록 해준다.
    여기서 참조자는 일종의 포인터같은 개념으로 참조자를 사용하는 함수 내에서 참조자는 함수 외부에 있는 원본 변수를 가리키고 있게 된다.
    러스트의 참조자는 C++의 참조자와 명칭도 그렇고 좀 비슷해 보이지만 C++의 참조자마냥 원본 변수의 값을 함부로 변경하는 것은 불가능하다.
    참조자로 빌려온 값은 함부로 변경할 수 없다. (근데 어차피 v_str_3 변수는 immutable하기 때문에 함수 외부에서도 변경할 수 없는건 마찬가지이다.)
    러스트의 참조자를 C++의 참조자처럼 함수 내부에서 값을 변경 가능하게 하려면 "가변 참조자"를 사용해야 한다.
    가변 참조자는 앤드 기호와 타입 혹은 변수 사이에 mut 키워드를 붙이면 된다.
    */
    let v_str_3 = String::from("string");
    let mut v_str_4 = String::from("string");
    let len3 = strlen(&v_str_3);
    println!("v_str_3 : {}", v_str_3);
    println!("v_str_3.len() : {}", len3);
    println!("v_str_4 (before) : {}", v_str_4);
    str_change(&mut v_str_4);
    println!("v_str_4 (after) : {}", v_str_4);

    /*
    참조자는 함수의 인수뿐만 아니라 변수를 선언할때에도 사용할 수 있다.
    일반 참조자(불변 참조자)는 여러개 만들어도 딱히 제한이 없다.
    근데 C같은 언어마냥 특정 변수에 대해 가변 참조자를 여러개 만들수는 없고 하나밖에 만들지 못한다.
    하나로 제한해둔 이유는 data race를 원천적으로 방지하기 위함이다.
    데이터 레이스는 아래 세가지 동작이 발생할 때에 대한 상황을 의미한다.
        1. 두개 이상의 포인터가 동시에 하나의 데이터에 접근한다.
        2. 1번의 상황에서 하나라도 데이터를 쓸 때
        3. 2번의 상황에 대해 동기화를 하는 매커니즘이 없을 때 발생한다.
    예를 들면 두개 이상의 스레드에서 하나의 변수에 접근하는 상황 등이 있을 것이다.
    불변 참조자와 가변 참조자를 구분함으로서 원본 변수가 불변인지 가변인지 바로 확인되는 이점도 있다.
    그리고 어떤 변수에 대해 불변 참조자를 만들었을 때 이후로 가변 참조자를 만들수는 없다.
    */
    let origin_str_immutable = String::from("string");
    let ref_str_immutable_1 = &origin_str_immutable;
    let ref_str_immutable_2 = &origin_str_immutable;
    let mut origin_str_mutable = String::from("string");
    let ref_str_mutable = &mut origin_str_mutable;
    println!("ref_str_immutable_1 : {}", ref_str_immutable_1);
    println!("ref_str_immutable_2 : {}", ref_str_immutable_2);
    println!("ref_str_mutable : {}", ref_str_mutable);

    /*
    소유권을 갖지 않는 것 중 슬라이스라는 것도 있다.
    아래의 hello, world 변수에 대괄호로 코드 붙여놓은것이 string 변수의 일부분에 대한 참조자이다.
    이는 스트링 슬라이스라고도 부른다.
    대괄호 내에 [starting_index..ending_index]로 특정한 범위에 대해 슬라이스를 만들 수 있다.
    ending_index는 슬라이스에 포함할 값의 인덱스보다 1이 커야 한다.
    starting_index와 ending_index는 생략 가능하며 생략하면 각각 0, max가 된다.
    여기서 hello와 world는 string의 동일한 힙 영역의 주소를 가리키고 있다.
    "스트링 슬라이스" 의 타입은 &str이다.
    */
    let string = String::from("hello world");
    let hello: &str = &string[0..5];
    let world = &string[6..string.len()];
    println!("part a : {}, part b : {}", hello, world);

    /*
    스트링 슬라이스는 원본 스트링 값과 그로부터 파생된 스트링들이 있을 때 동기화를 용이하게 한다.
    슬라이스는 원본 값이 파생된 값이 접근 못하게 변경이 된다면 컴파일 타임에 오류가 발생한다.
    아래에서 word가 정해지고 난 이후로 string_origin가 변동되고 나서 word를 참조해선 안된다. (컴파일 에러)
    그래서 스트링 슬라이스는 원본 문자열이 불변임을 보증해준다.
    */
    let mut string_origin = String::from("hello world");
    let word = get_first_word(&string_origin);
    println!("string_origin({})'s first word : {}", string_origin, word);
    string_origin.push_str(" ㅎㅎ");
    println!("string_origin : {}", string_origin);
    //println!("first word : {}", word);

    /*
    이 소스코드 내에서 맨 처음에 다루었던 스트링 리터럴의 타입은 스트링 슬라이스이다.
    바이너리의 특정 위치를 가리키는 스트링 리터럴은 불변이므로 불변 참조자인 &str 타입을 가진다.
    */
}

fn get_integer(var: i32) {
    println!("var : {}", var);
}

fn get_string(var: String) {
    println!("var : {}", var);
} // 여기서 내부적으로 var 변수에 대해 drop을 호출하여 힙 영역 해제한다.

fn strlen_bad(var: String) -> usize {
    var.len()
}

fn strlen_fix(var: String) -> (String, usize) {
    let len = var.len();

    (var, len) // 이렇게 더러운 방법을 사용하는 방법도 있다.
}

fn strlen(var: &String) -> usize {
    // 여기서 var은 외부에서 호출한 변수의 주소를 가지고 있게 된다.
    var.len()
}

fn str_change(var: &mut String) {
    // 가변 참조자로 인수를 받아 입력된 변수를 변경함
    var.push_str("_string");
}

fn get_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}