fn main() {
    /*
    러스트에서는 C++나 자바같은데서 지원하는 예외 처리 기능을 지원하지 않는다.
    그래서 에러가 발생할만한 상황에 대해서 수동으로 직접 처리하여야 한다.

    에러가 발생할때 어쩔 수 없이 프로그램을 종료할때 panic! 매크로로 메시지를 출력하고 종료한다.
    저 매크로로 종료할 때 보통 메시지 출력 -> 스택 되감기 -> 종료 순서로 종료되는데
    스택 되감기는 스택을 되감으며 데이터를 직접 제거하는 것이라 처리해야 하는 일이 좀 있다.
    그래서 데이터를 청소하지 않고 즉시 종료하게 하려면
    Cargo.toml 파일의 [profile] -> panic = 'abort'를 추가해야 한다.

    예를 들면 다음과 같이 기입한다.
    [profile.release]
    panic = 'abort'

    panic! 매크로가 호출되면 프로그램은 즉시 종료된다.
    */
    //panic!("Hello, crash!");

    /*
    panic! 매크로는 사용자가 명시적으로 호출하지 않아도 코드의 버그때문에 호출될 수도 있다.
    예를 들면 아래와 같은 경우가 있다. 아래와 같은 에러를 의도적으로 발생시키면
    panic!("index out of bounds: the len is 3 but the index is 99")를 호출한 것과 비슷한 에러가 뜬다.
    참고로 아래와 같은 문제는 "버퍼 오버리드 (buffer overread)" 라고 한다.

    오류가 발생되면 "note: Run with `RUST_BACKTRACE=1` for a backtrace." 라는 메시지가 뜨는데
    프로그램을 실행할 때 저 환경변수를 저렇게 쓰면 백트레이스를 출력해준다.
    백트레이스를 출력하면 오류가 어디서 발생했는지 정확하게 확인할 수 있다.
    백트레이스는 아래에서 위 순서로 읽는다.
    프로그램을 빌드할 때 그냥 빌드하면 함수의 심볼정보가 바이너리에 추가되는데 릴리즈를 할 때에는 함수의 심볼점보를 제외하는게 보통이다.
    그래서 --release 옵션을 붙여서 빌드하면 백트레이스에서 출력되는 정보가 줄어든다.
    */
    let v = vec![1, 2, 3];
    println!("v[0] : {}", v[0]);
    //println!("v[99] : {}", v[99]);

    /*
    프로그램 상에서 발생하는 에러는 여러 종류가 있는데 프로그램을 종료해야 할 정도로 크리티컬하지 않으면
    보통 프로그램 내에서 처리해야 한다.

    러스트에서 흔히 사용하는 Result 열거형 타입은 결과가 성공할 때, 실패할 때를 다룰 수 있도록 해주는데
    예를 들어서 파일을 여는 작업은 성공할 수도 있으며 실패할 수도 있다. 이 작업을 예로 들어 Result가 리턴될 때
    에러 핸들링을 어떻게 하는지 살펴보자.

    어떤 작업을 할 때 값을 반환하는지 Result를 반환하는지 혼동이 될 수 있는데
    컴파일러한테 물어보거나 (컴파일 에러로 확인) API 문서를 확인하는게 보통이다.

    이전에 말했듯이 Result 열거형 타입은 러스트 표준 라이브러리에 정의되어 있지만
    러스트는 (아마도 편리한 사용을 위해) 기본으로 import 해둔다.

    러스트는 이러한 항목들을 std::prelude에 정의해놓았으며
    이 항목들은 별도로 import 하지 않아도 바로 사용이 가능하다.

    모든 에러는 Err를 이용해 핸들링할 수 있지만 에러를 특정지을수가 있다.
    예를 들면 파일을 열 때 발생 가능한 에러는 많다. 파일이 존재하지 않거나 권한이 없거나...
    원한다면 이런 경우에 대해서 모두 특정지을수 있다.

    match 내에서 error에 대해 & 대신 ref가 사용되는데 &는 참조자를 매칭하지만 ref는 값을 매치하기 때문이다.
    자세한건 나중에 다룬다.
    */
    use std::fs::File;
    use std::io::ErrorKind; // 입출력 연산으로부터 발생할 수 있는 에러 종류가 정의되어 있다.
    let file = File::open("file.txt");
    
    let file = match file {
        Ok(file) => file,
        // 만약 파일이 존재하지 않는 에러라면
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            // match 안에 파일 생성하는 함수를 넣을 수도 있다. 위의 파일 open 하는 함수를 이렇게 해도 된다.
            match File::create("file.txt") {
                Ok(file_create) => file_create,
                Err(create_error) => {
                    panic!("파일 생성 실패 ({:?})", create_error);
                }
            }
        }
        Err(error) => {
            panic!("파일 열기 실패 ({:?})", error);
        }
    };

    /*
    위처럼 장황하게 작성하면 코드가 너무 길어지는 단점이 있다.
    그래서 Result 타입에 대해 동작하는 많은 "헬퍼 메소드"들이 있으며
    예를 들면 unwrap과 expect이라는 메소드가 있다.
    https://doc.rust-lang.org/std/result/enum.Result.html 에 보면 Result에 대해 동작하는 수십개의 메소드들이 있다.

    unwrap 메소드는 Result 타입의 리턴값이 Ok이면 Ok 내의 값을 반환하며
    Err라면 panic! 매크로를 호출한다.

    expect 메소드는 panic! 메시지를 직접 선택할 수 있도록 해준다.
    사용자가 직접 입력한 메시지와 error 메시지가 같이 나온다.
    */
    //let file_unwrap = File::open("file_unwrap.txt").unwrap();
    //let file_expect = File::open("file_expect.txt").expect("파일 열기 실패");

    /*
    C++나 자바같은 언어에서 에러가 발생할지도 모르는 함수 호출을 try ~ catch 문으로 감싸는것처럼 처리하는 대신
    러스트에서는 에러가 발생할지도 모르는 함수에서 에러가 발생하면 Result 형을 리턴해 에러를 반환하도록 한다.
    이를 "에러 전파하기" 라고 부른다.

    에러를 전파할 때 물음표 (?) 연산자를 사용해 간단하게 짤 수 있다.
    Result가 반환되는 함수의 뒤에 물음표 연산자를 붙이면 Ok 리턴시 Ok 내 값이 표현식으로 얻어지며
    Err 리턴시 Err를 바로 리턴한다.

    그렇기 때문에 ?는 Result를 리턴하는 함수 내에서만 사용할 수 있다.
    */
    //let file_contents = read_line_from_file("Cargo.toml").expect("파일 열기 실패");
    let file_contents = read_line_from_file_short("Cargo.toml").expect("파일 열기 실패");
    println!("file : {}" , file_contents);
}

fn read_line_from_file(filename: &str) -> Result<String, std::io::Error> {
    use std::io::Read; // read_to_string 메소드를 사용하기 위함

    let mut file = match std::fs::File::open(filename) {
        Ok(file) => file,
        Err(error) => return Err(error)
    };

    let mut rtn_str = String::new();
    
    match file.read_to_string(&mut rtn_str) {
        // 각각 상황에 맞게 Result 타입으로 감싸서 리턴하게 됨
        Ok(_) => Ok(rtn_str),
        Err(error) => Err(error)
    }
}

fn read_line_from_file_short(filename: &str) -> Result<String, std::io::Error> {
    use std::io::Read; // read_to_string 메소드를 사용하기 위함

    let mut file = std::fs::File::open(filename)?;

    let mut rtn_str = String::new();
    
    file.read_to_string(&mut rtn_str)?;
    Ok(rtn_str)
}
