// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 5;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 2 {
//                 break;
//             }
//             if count == 3 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }

//         count += 1;
//     }
//     println!("End count = {count}");
// }

// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     let mut index = 0;

//     while index < 5 {
//         println!("the value i: {}", a[index]);

//         index += 1;
//     }
// }

// fn main() {
//     let a = [1, 2, 3, 4, 5];

//     for el in a {
//         println!("the value is: {el}")
//     }
// }

fn main() {
  for num in (1..4).rev() {
      println!("{num}!");
  }
  println!("LIFTOFF!!!");
}
