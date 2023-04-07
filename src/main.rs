use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_num = rand::thread_rng().gen_range(1..=100);

    // 비밀번호 출력 줄 삭제
    // println!("The secret number is: {secret_num}");

    // 아래의 루프를 반복함.
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // guess 이전의 값을 shadowing함. guess: u32는 변수 유형에 주석을 달 것을 알림. ㅕ32는 부호가 없는 32비트 정수.
        let guess: u32 = match guess
            // 시작과 끝의 공백 제거. 엔터 키를 눌러 guess를 입력할 때, \n이 추가되므로 이를 trim 메서드로 제거함.
            .trim()
            // 다른 유형으로 변환함. 
            .parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            // 숫자가 아닌 유형이 입력되어 파싱에 실패했을 때의 기대출력값.
            // .expect("Please type a number!");

        println!("You guessed: {guess}");

        // cmp() => guess와 참조된 secret_num을 비교한다.
        match guess.cmp(&secret_num) {
            // Ordering 유형은 열거형으로 Less, Greater, Equal 변형이 있다.
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("That's right! You win!");
                break;
            }
        }
    }
}
