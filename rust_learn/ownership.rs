// 소유권으로 인해 가비지 컬렉터 없이 메모리 안전성을 보장할 수 있음.

// 소유권 규칙
// 1. Rust의 각 값에는 소유자가 있다.
// 2. 소유자는 한 번에 한 명만 있을 수 있다.
// 3. 소유자가 범위를 벗어나면 해당 값은 삭제된다.

// // 참조 오류 발생. 
// fn main() {
//   let s1 = String::from("hello");
//   let s2 = s1;

//   println!("{}, world!", s1);
// }

// 깊은 복사를 원한다면? clone 메서드 사용
// fn main() {
//   let s1 = String::from("hello");
//   let s2 = s1.clone();

//   println!("s1 = {}, s2 = {}", s1, s2);


//   // 정수형과 같이 컴파일 타임에 결정되어 있는 크기의 타입은 스택에 모두 저장됨.
//   // 실제 값의 복사본이 빠르게 만들어질 수 있음.
//   let x = 5;
//   let y = x;

//   println!("X = {}, y = {}", x, y);
// }

// copy가 가능한 몇가지 타입
// 1. u32 와 같은 모든 정수형 타입들
// 2. true 와 false 같은 값을 갖는 불리업 타입
// 3. f64 와 같은 모든 부동 소수점 타입들
// 4. copy가 가능한 타입만으로 구성된 튜플들. (i32, i32)는 copy가 되지만, (i32, String)은 안 됨.

// 소유권과 함수
// fn main() {
//   let s = String::from("hello"); // s가 스코프 안으로 들어옴

//   takes_ownership(s); // s의 값이 함수 안으로 이동
//                       // 더이상 유효하지 않음
//   let x = 5;          // x가 스코프 안으로 들어옴
  
//   makes_copy(x);      // x가 함수 안으로 이동했으나 i32는 copy가 되므로 x를 이후에 계속 사용할 수 있음
// }                     // 여기서 x는 스코프 밖으로 나가고, s도 나감. 하지만 s는 이미 이동되어 다른 일이 발생하지 않음


// fn takes_ownership(some_string: String) { // some_string scope in
//   println!("{}", some_string);            // some_string scope out, call 'drop', memory freed
// }

// fn makes_copy(some_integer: i32) { // some_integer scope in
//   println!("{}", some_integer);
// }                                  // some_integer scope out, call 'drop', memory freed

// fn main() {
//   let s1 = gives_ownership();         // gives_ownership은 반환값을 s1에게
//                                       // 이동시킵니다.

//   let s2 = String::from("hello");     // s2가 스코프 안에 들어왔습니다.

//   let s3 = takes_and_gives_back(s2);  // s2는 takes_and_gives_back 안으로
//                                       // 이동되었고, 이 함수가 반환값을 s3으로도
//                                       // 이동시켰습니다.

// } // 여기서 s3는 스코프 밖으로 벗어났으며 drop이 호출됩니다. s2는 스코프 밖으로
// // 벗어났지만 이동되었으므로 아무 일도 일어나지 않습니다. s1은 스코프 밖으로
// // 벗어나서 drop이 호출됩니다.

// fn gives_ownership() -> String {             // gives_ownership 함수가 반환 값을
//                                            // 호출한 쪽으로 이동시킵니다.

//   let some_string = String::from("hello"); // some_string이 스코프 안에 들어왔습니다.

//   some_string                              // some_string이 반환되고, 호출한 쪽의
//                                            // 함수로 이동됩니다.
// }

// // takes_and_gives_back 함수는 String을 하나 받아서 다른 하나를 반환합니다.
// fn takes_and_gives_back(a_string: String) -> String { // a_string이 스코프
//                                                     // 안으로 들어왔습니다.

//   a_string  // a_string은 반환되고, 호출한 쪽의 함수로 이동됩니다.
// }

fn main() {
  let s1 = String::from("hello");

  let (s2, len) = calculate_length(s1);

  println!("The length of '{}' is {}", s2, len);
}

// -> 뒤에는 반환 값의 타입을 선언해 줌. rust에서 반환 값은 함수 본문의 마지막 표현식의 값과 동일하다. (s, length)
fn calculate_length(s: String) -> (String, usize) {
  let length = s.len();

  (s, length)
}