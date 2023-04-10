// fn main() {
//   let s1 = String::from("hello");

//   // &s1 문법은 s1의 값을 참조하지만 소유하지는 않는 참조자를 생성함  
//   let len = calculate_length(&s1);

//   println!("The length of '{}' is {}", s1, len);
// }

// // String s1을 가리키고 있는 &String s
// fn calculate_length(s: &String) -> usize {
//   s.len()
// }

// 참조로 빌린 값은 변경할 수 없음
// mutable reference로 바꾸면 변경이 가능함
// 그러나 가변 참조자는 스코프 내의 특정 데이터 조각에 대한 가변 참조자를 딱 하나만 만들 수 있음
// fn main () {
//   // let s = String::from("hello");
//   let mut s = String::from("hello");

//   // change(&s);
//   change(&mut s);
// }

// // fn change(some_string: &String) {
// fn change(some_string: &mut String) {
//   some_string.push_str(", world");
// }

// 중괄호로 스코프를 하나 더 생성해주면 가변참조자를 또 만들 수 있음
// fn main () {
//   let mut s = String::from("hello");
  
//   {
//     let  r1 = &mut s;
//   }
  
//   let r2 = &mut s;
// }

// dangling references
fn main() {
  let _reference_to_nothing = dangle();
}

// fn dangle() -> &String {
fn dangle() -> String {
  let s = String::from("hello");

  // lifetime 에러 발생
  // &s
  // s를 직접 반환하면 문제 해결
  s
}