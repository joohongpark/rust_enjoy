# test 함수와 assert 매크로

## 테스트 함수

러스트에서 말하는 테스트는 `test` 라는 속성(attribute)이 달려있는(annotated) 함수를 말한다. 속성은 코드에 대한 메타데이터를 의미한다.

`cargo test` 를 실행시키면 `test` 속성이 달려있는 함수들을 실행하게 하며 테스트용 바이너리를 생성하고 실행시킨다. 그리고 이 함수들의 성공과 실패 여부를 알려준다.

cargo로 라이브러리를 만들면 기본으로 테스트 모듈을 생성해 준다.

## 테스트 함수 써보기

### 라이브러리 프로젝트 생성

```shell
$> cargo new --lib --vcs=none libtest
```

### src/lib.rs 내용

기본으로 생성되는 코드는 다음과 같다.

```rust
#[cfg(test)] // 추후에 설명
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

함수 it_works에 test라는 속성이 달려 있으므로 it_works는 테스트 함수이다. 함수 내에는 매크로 `assert_eq!` 는 메소드의 인수 두개가 같은지 아닌지를 판별한다.

이 코드에 대해 테스트들을 실행시키는 명령은 다음과 같다.

```shell
$> cargo test
```

테스트 명령을 실행하면 쉘에 컴파일하고 실행했다는 내용이 뜨며 몇개의 테스트를, 어떤 테스트를 실행했는지에 대한 짧은 보고가 뜬다. 이 실행결과는 cargo가 만들어낸 바이너리를 실행시킨 것이므로 쉘에 나타나는 Running 뒤에 나오는 파일을 실행시키면 동일한 결과가 뜬다.

결과로 나오는 각각의 테스트 이름은 테스트 함수명이 된다.

### 다른 테스트 추가 / panic! 매크로

테스트 하고자 하는 함수를 만들어 테스트 어노테이션 `#[test]를 함수에 붙이면 테스트 함수가 된다.

테스트 함수 내에선 panic이 발생하면 그 테스트는 실패한 것으로 간주하므로 `panic!` 매크로가 실행된다면 테스트는 실패한다.

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another_test() {
        panic!("테스트 실패");
    }
}
```

이 테스트를 실행시키면 it_works는 성공하지만 another_test는 실패하고 어느 부분에서 테스트를 실패했는지 알려준다.

```
[중략]
test tests::another_test ... FAILED

failures:

---- tests::another_test stdout ----
thread 'tests::another_test' panicked at '테스트 실패', src/lib.rs:10:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[중략]
```

메시지까지 출력해 주므로 한 테스트 함수 내에 여러 조건들이 있고 실패하는 조건들이 다 다를때 panic! 매크로를 사용하면 좋다.

### assert! 매크로

assert! 매크로는 인수가 false이면 에러를 발생시킨다. 선택사항으로 커스텀 메시지를 인수로 추가할 수 있다.

tests는 모듈이고 모듈 외부에 임의로 정의한 is_odd에 접근하기 위해 tests 내 스코프로 가져오기 위한 설정을 하였다.

is_odd 라는 함수를 public으로 선언하여 다른 모듈 내에서도 정상적으로 접근할 수 있도록 하였다.

```rust
pub fn is_odd(arg: i32) -> bool {
    arg % 2 == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_test() {
        assert!(is_odd(1));
    }

    #[test]
    fn two_test() {
        assert!(is_odd(2));
    }

    #[test]
    fn one_test_msg() {
        assert!(is_odd(1), "{} is not odd", 1);
    }

    #[test]
    fn two_test_msg() {
        assert!(is_odd(2), "{} is not odd", 2);
    }
}
```

### assert_eq!와 assert_ne! 매크로

두 매크로는 두 인수가 같은지 혹은 같지 않은지를 판별하는 매크로이다.

만약 두 값이 같다면 (혹은 같지 않다면) 에러를 발생시키며 이 매크로들은 두 인자들을 디버그 포맷팅을 사용해서 출력해준다.

그래서 `PartialEq` 트레잇과 `Debug` 트레잇이 구현되어 있어야 사용이 가능하며 표준 라이브러리의 타입들은 이 타입들이 구현되므로 정상적으로 사용 가능하다.

새로운 타입을 정의해서 비교한다면 두 트레잇이 구현되어 있어야 한다.

```rust
pub fn is_odd(arg: i32) -> bool {
    arg % 2 == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_test() {
        assert_eq!(is_odd(1), false);
    }

    #[test]
    fn two_test() {
        assert_eq!(is_odd(2), false);
    }
}
```

## should_panic 속성

테스트 함수에서 에러가 반드시 발생해야 하는 상황도 있을것이다. 에러가 발생하지 않는 상황이 에러인 것이다. 이런 상황을 체크하려면 테스트 함수에 should_panic 속성을 추가하면 된다.

```rust
pub struct Guess {
    value: u32
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        let _g = Guess::new(200);
    }

    #[test]
    fn lower_than_100() {
        let _g = Guess::new(99);
    }
}
```

이 코드를 실행하면 should_panic이 어떻게 동작되는지 확인할 수 있다.

## 테스트 바이너리

`cargo test` 는 테스트 모드로 컴파일하고 바이너리를 실행하는 명령이다. 그래서 실행하는 바이너리에 옵션을 넣을 수 있다.

하지만 옵션을 넣을 때 cargo가 인식하는 옵션과 바이너리가 인식하는 옵션을 구분할 필요가 있는데 `--` 구분자를 사용하여 다음과 같이 구분한다.

```shell
$> cargo test [cargo 옵션] -- [바이너리 옵션]
```

테스트 바이너리는 여러 옵션을 가지고 있으며 이는 `--help` 옵션으로 확인할 수 있다.

### 테스트 바이너리의 스레드

테스트 바이너리는 기본적으로 멀티스레드로 동작한다. 그래서 한 순간에 테스트가 여러개 실행되고 있을 수 있으며 만약 테스트 항목들이 서로 간에 영향을 미칠 우려가 있다고 판단하면 테스트 바이너리의 옵션 `--test-threads` 을 이용해 제한할 수 있다.

다음 테스트 명령은 테스트가 실행될 때 하나의 스레드로 동작하게 제한한다. (대신 시간은 더 오래 걸린다.)

```shell
$> cargo test -- --test-threads=1
```

### 성공한 테스트의 표준출력 캡쳐 여부

테스트 바이너리는 실패한 테스트의 경우 표준출력으로 출력되는 내용을 출력하지만 성공한 테스트의 경우 표준출력을 생략한다. (capture)

사용자가 원하면 성공한 테스트의 표준 출력을 볼 수 있으며 텍스트 바이너리에 `--nocapture` 플래그를 사용하면 볼 수 있다.

### 특정 테스트 (함수) 만 실행하기

인수에 실행하고자 하는 테스트명을 적으면 그 테스트만 실행된다. 이 방법으로는 단 하나의 테스트만 실행할 수 있다.

```shell
$> cargo test [테스트명]
$> cargo test -- [테스트명]
```

두가지 방법 모두 가능하다.

### 특정 이름을 포함하고 있는 테스트만 실행하기

필터처럼 특정 문자열을 가지고 있는 테스트만 실행하는 법이다. 실행방법은 위와 같다.

```shell
$> cargo test [테스트명의 부분문자열]
$> cargo test -- [테스트명의 부분문자열]
```

### 테스트 함수의 ignore 속성과 --ignored 옵션

테스트 함수에 `ignore` 속성을 추가하면 테스트 목록에서는 감지하지만 테스트를 수행하지는 않는다. 보통 시간이 오래 걸리거나 하는 테스트에 이 속성을 추가하며 이 테스트를 실행시키려면 테스트 바이너리에 `--igonred` 옵션을 넘긴다.

## 단위 테스트(unit test)와 통합 테스트(integration test)

단위 테스트는 비공개 인터페이스, 작은 하나의 모듈을 테스트하며 통합 테스트는 라이브러리 외부에서 여러 개의 모듈을 잠재적으로 테스트한다.

### 단위 테스트

단위 테스트는 모듈을 다른 부분과 분리하여 테스트 하는 것이며 이것은 코드상에 문제가 있을 시에 해당 부분을 정확하게 찾는 것에 도움을 준다.

단위 테스트는 src 경로에 있으며 파일마다 테스트를 수행하는 코드를 담고 있다.

관례적으로 테스트 파일마다 테스트 함수를 담고 있는 `tests` 라는 모듈을 만들고 이 모듈에 `cfg(test)` 를 어노테이션 하는 것이다.

### cfg(test) 어노테이션

cargo로 새로운 라이브러리를 만들면 위에서 말한 관례적인 테스트 파일을 만들어 준다.

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

cfg(test) 어노테이션의 cfg() 는 환경 설정의 줄임말이며 어노테이션 된 대상이 괄호 내의 대상에만 포함되어야 한다는 것을 의미한다.

그래서 cargo로 test 명령어를 이용해 테스트할 때에만 저 어노테이션이 붙어있는 모듈이 활성화되어 컴파일되는 것이다.

### 비공개 함수 테스트

일반적으로 비공개 함수는 테스트할 필요 없다는 의견이 대다수이다. 하지만 러스트는 비공개 함수를 테스트 할 수 있도록 허용한다.

```rust
fn is_odd(arg: i32) -> bool {
    arg % 2 == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_test() {
        assert!(is_odd(1));
    }
}
```

위에서 is_odd에 `pub` 키워드가 빠지면 private 속성을 띄게 되는데 그런 경우 is_odd 함수는 비공개 함수가 된다. 하지만 러스트는 그런 경우에도 테스트를 수행하도록 허용해준다.

### 통합 테스트

통합 테스트에서는 라이브러리의 공개 API로만 접근하고 호출할 수 있다.

통합 테스트 코드는 src와 별도의 라이브러리에 만들며 `tests` 라는 폴더 내에 만든다. 디렉토리를 만들어 두고 `cargo test` 를 실행하면 cargo가 알아서 통합 테스트 파일을 찾아서 컴파일 해 준다.

tests 폴더 내의 테스트 코드는 다음과 같이 만든다.

```rust
extern crate adder;

#[test]
fn integral_test() {
    let _g = adder::Guess::new(99);
}
```

- 단위 테스트랑 다르게 tests 모듈을 만들고 어노테이션을 추가할 필요가 없다.
  - cargo는 tests 폴더 내의 내용물을 특별 취급해 `cargo test` 명령을 내릴 때에만 컴파일한다.
- 동일한 모듈 내에서 테스트할때에도 src 내에 있는 코드를 크레이트로서 가져와야 한다.
  - tests 내의 테스트 파일들은 개별적인 크레이트로 식별되기 때문이다.

cargo test를 실행하면 단위 테스트 -> 총합 테스트 순으로 테스트 한다.

### 통합 테스트 내 테스트를 위한 서브 모듈

통합 테스트를 수행할 때 여러 테스트 함수들이 공통적으로 수행할 코드를 담아둔 서브 모듈을 만들고 싶을 때는 tests 폴더 내에 공통으로 수행할 코드들을 `[폴더명]/mod.rs` 내에 집어 넣는다.

그런 다음 모듈을 사용하고자 하는 통합 테스트 코드 내에 `mod [폴더명]`  을 집어 넣어 모듈을 추가한다.

이런 식으로 사용해야 서브 모듈들이 테스트 코드로 동작하는 것을 방지할 수 있다.

예를 들면 테스트 코드를 위한 공통 코드(특정 숫자를 얻기 위한 함수를 만든다면)를 `tests/comm/mod.rs` 내에 다음과 같이 작성한다.

```rust
pub fn get_num() -> u32 {
    1234
}
```

이후 통합 테스트 내에서는 다음과 같이 함수를 호출한다.

```rust
extern crate adder;

mod comm;

#[test]
fn integral_test() {
    let num = comm::get_num();
    assert_eq!(num, 1234);
}
```

## 바이너리 크레이트와 통합 테스트

`cargo new --bin` 으로 생성된 프로젝트는 바이너리 크레이트에 속한다.

바이너리 크레이트에서는 통합 테스트를 위한 tests 내의 테스트 코드에서 `extern crate` 를 허용하지 않는다. 왜냐 하면 바이너리 크레이트는 스스로 실행되는 것으로 여겨지기 때문이며 또 그럴 필요도 없기 때문이다.

