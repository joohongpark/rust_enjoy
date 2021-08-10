/*
모듈의 정의
모듈의 정의는 모두 mod로 시작된다. mod 뒤에 오는것은 모듈 이름이다.
모듈 내의 모든 것들은 모듈 이름 네임스페이스 안에 있게 된다.
그래서 모듈 내의 것을 모듈 외부에서 호출하려면 네임스페이스 문법 ::을 붙여야 한다.
한 파일 내에 여러 모듈을 놓아도 상관없으며 모듈 안에 다른 모듈을 넣어도 상관없다.

모듈의 요소에는 가시성(visibility) 라는게 있다.
모듈이나 모듈 내의 특정 요소가 비공개(private)라면 같은 파일 내에 있는 부모 모듈이나 자식 모듈에만 접근 가능하며
특정 요소가 공개(public)라면 부모 모듈의 어디에서건 접근이 가능하다.
보통 기본값은 private이며 pub 키워드를 붙이면 퍼블릭으로 바뀐다.
*/
mod network_test {
    fn connect() {
        //
    }
    mod server_test {
        fn connect() {
        }
    }
}
mod client_test {
    pub fn connect() {
    }
}

/*
모듈을 파일로 분할
라이브러리 크레이트의 경우 기본적으로 src/lib.rs 파일만을 찾아본다.
그래서 모듈을 다른 파일로 분리할 때 선언부만 남겨두면 src 내 다른 경로의 모듈을 찾아준다.
다른 경로에 모듈을 만들 때 중요한 규칙이 있다.
1. 만약 A라는 이름을 가진 모듈이 서브모듈이 없다면 A.rs라는 파일 내에 모듈 본문을 집어 넣는다.
2. 만약 B라는 이름을 가진 모듈이 서브모듈이 있다면 B/mod.rs 라는 파일에 B의 선언을 집어 넣는다.
*/
pub mod network;
pub mod client;

/*
러스트에서는 겹겹이 구성된 모듈에서 함수를 호출할 때 네임스페이스 영역이 길어지는 것을 간결하게 해 주는 키워드가 있다.
예를 들어 아래의 nested_modules 함수를 호출하려면 a::series::of::nested_modules(); 와 같이 호출해야 하지만
use 키워드를 사용하면 스코프를 생략하게 될 수 있다.
예를 들어 use a::series::of; 로 선언하면 of::nested_modules();와 같이 함수를 호출할 수 있다.
이는 네임스페이스가 형성되는데서 공통적으로 사용할 수 있다.
자바처럼 * (glob) 을 사용해 특정 네임스페이스 내의 예하에 있는 모든 아이템을 가져오게 할 수도 있지만
이러면 네이밍 충돌이 발생할 가능성이 있으므로 신중하게 사용하여야 한다.
*/
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}
/*
이 프로젝트는 라이브러리 크레이트이다.
바이너리가 아니므로 main 함수가 없으며 처음 cargo로 생성시 빈 테스트(mod tests)를 만들어 준다.
main.rs를 추가해서 실행 테스트를 해도 된다. 딱히 상관 없다.
이런 코드가 있다면 cargo test 명령어를 이용해 코드 동작 테스트를 시켜준다.

아래에 보면 이 라이브러리에 정의된 모듈을 참조할 때 super 키워드를 사용했는데
super 키워드의 역할은 계층 구조 상 현재 위치에서 상위 모듈로 거슬러 올라가는 것을 의미한다.
왜 super 키워드를 사용해야 하냐면 밑 테스트도 결국 모듈 내에 있어서 별도로 명시를 해야 하기 때문이다.
*/
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        super::client_test::connect();
        super::network::connect();
        super::network::server::connect();
    }
}

