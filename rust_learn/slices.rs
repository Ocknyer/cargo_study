fn first_word(s: &String) -> usize {
  // as_bytes()는 String을 바이트 배열로 반환함
  let bytes = s.as_bytes();

  // iter()는 바이트 배열의 반복자를 생성함
  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return i;
    }
  }

  s.len()
}

fn main() {
  let mut s = String::from("hello world");

  let word = first_word(&s);

  word.len();
  // s.clear(); //s를 비워 ""로 만들게 함
}