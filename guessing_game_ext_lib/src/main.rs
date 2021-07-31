extern crate rand; // 의존성이 있는 외부 crate 추가. use rand로 표기해도 된다고 함

use std::io;
use std::cmp::Ordering;
use rand::Rng; // 점수 생성기가 구현한 메소드들을 정의한 trait (trait가 무엇인지는 나중에)

fn main() {
    let rand_number = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("숫자를 입력하세여");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("라인 가져오기 실패");
        println!("[생성된 랜덤 숫자는 {} 입니다.]", rand_number);
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        println!("님이 입력한 숫자는 {} 입니다.", guess);
        match guess.cmp(&rand_number) {
            Ordering::Less      => println!("너무 작음"),
            Ordering::Greater   => println!("너무 큼"),
            Ordering::Equal     => {
                println!("님이 맞춤");
                break;
            }
        }
    }
    /*
    cmp 메소드는 비교가 가능한 모든것들에 대해 호출할 수 있다고 한다. cmp 메소드는 비교하고 싶은 것들의 참조자를 받는다.
    cmp는 Ordering 타입의 열거형을 리턴하기 때문에 use std::cmp::Ordering;을 사용하였다.
    match 표현문을 이용해 Ordering의 값에 따라 무엇을 할지 정할수 있다.

    match 표현식은 arm으로 이루어져 있다고 한다.
    하나의 arm은 하나의 패펀과 match 표현식에서 주어진 값이 패턴과 맞을때 실행할 코드로 이루어져 있다.
    match에 대한 것은 추후에 자세히 다룬다.

    러스트는 정적 타입 시스템을 가지고 있지만
        - 8줄, 10줄은 각 타입이 명확하기 때문에 타입을 적지 않는다.
    타입을 추론이 필요할 경우 동적으로 추론한다.
    하지만 위 stdin을 통해 입력한 데이터는 문자열 형이고 랜덤하게 생성한 데이터는 숫자이다. 그래서 별도로 형변환을 해야 한다.

    문자열 타입 guess는 trim, parse 메소드를 가지고 있다.
    trim 메소드는 처음과 끝의 빈칸 혹은 개행을 제거한다.
    parse 메소드는 문자열을 숫자로 파싱한다. 하지만 이 메소드는 다양한 형태의 숫자로 반환할 수 있으므로 타입을 명시할 필요가 있다.
    이 예제에선 변수를 선언할 때 타입을 정하는 것으로 하였다. 변수명 뒤 콜론은 변수의 타입을 명시한 것이다. (unsigned 32bit)

    parse 메소드는 예외가 발생할 가능성이 높다. 문자열 내 숫자가 있지 않을 가능성도 있기 때문이다.
    parse 메소드의 리턴값도 실패할 경우를 위해 io:Result 타입을 리턴해준다.
    그래서 expect 메소드를 실행해도 되고 match 표현식으로 처리해도 된다. (보통 match 표현식으로 처리한다.)
    저 이상한 모양새가 동작할 수 있는 이유는 parse 메소드가 오류가 발생하지 않았다면 Result 타입은 Ok를 반환하고 이어서 num을 guess에 넣는다.
    Err과 매칭된다면 (인수의 언더바는 모든값과 매칭될 수 있다. 따라서 모든 에러를 핸들링할 수 있다.) continue 키워드가 동작되 루프의 처음으로 돌아간다.
    
    loop 키워드는 while (1)과 동일하다. 내부에서 break 키워드로 빠져나오기 전까지 무한반복을 수행한다. continue 키워드를 쓸수도 있다.

    결과적으로 우리는 let, match, 메소드, 연관함수, 외부 크레이트와 같이 러스트의 기능들을 사용해 보았다.
    */
}
