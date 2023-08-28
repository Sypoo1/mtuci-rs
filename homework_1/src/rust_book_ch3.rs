// use std::io;

// fn main() {
//     let a = [1, 2, 3, 4, 5];

//     println!("Please enter an array index.");

//     let mut index = String::new();

//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line");

//     let index: usize = index
//         .trim()
//         .parse()
//         .expect("Index entered was not a number");
//     if index <= 4 {
//         let element = a[index];
//         println!("The value of the element at index {index} is: {element}");
//     } else {
//         println!("The value of the element at index {index} is not defined");
//     };

// }



// fn main() {
//     let t = ([1; 2], [3; 4]);
//     let (a, _) = t;
//     println!("{}", a[0] + t.1[0]); 

//     println!("{} {} {}", a[1], t.1[1], t.1[2]);
//   }



// fn above_main() {
//     println!("hello from above_main fn");
// }

// fn main() {
//     println!("hello from main fn");
//     not_main();
//     above_main();
// }

// fn not_main() {
//     println!("hello from not_main fn");
// }

// fn main() {
//     print_labeled_measurement(5, 'h');
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }

// // need a type
// fn f(x:i32) { 
//     println!("{x}");
//   }
//   fn main() {
//     f(0);
//   }



// // Listing 3-1: A main function declaration containing one statement
// fn main() {
//     let y = 6;
//     // println!("hi")
// }



// fn main() {
//     // let x = (let x = 6);
//     let x:i32 = {
//         let x: i32 = 5;
//         x + 1
//     };
//     println!("{x}")
// }



// fn six() -> i32 {
//     6 // or return 6;
// }

// fn main() {
//     let x:i32 = six();

//     println!("The value of x is: {x}");
// }



// fn main() {
//     let x: i32 = plus_one(5);
//     println!("{x}");
// }

// fn plus_one(value: i32) -> i32 {
//     return value + 1;
// }



// fn main() {
//     let x:i32 = plus_one(5);

//     println!("The value of x is: {x}");
// }

// fn plus_one(x: i32) -> i32 {
//     x + 1
// }




// fn f(x: i32) -> i32 {
//      x + 1
// }
// fn main() {
//   println!("{}", f(
//     {
//         let y = 1;
//         y + 1
//     }
//     )
// );
// }


// fn main() {
//     // let number = 5;
//     let number = 2;
//     if number > 3{
//         println!("True")

//     } else {
//         println!("False")
//     }
// }



// fn main() {
//     let number = 6;

//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2");
//     } else {
//         println!("number is not divisible by 4, 3, or 2");
//     }
// }

// fn main() {
//     let number = 6;

//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     }
//     if number % 3 == 0 {
//         println!("number is divisible by 3");
//     }
//     if number % 2 == 0 {
//         println!("number is divisible by 2");
//     }
// }


// fn main() {
//     // let condition: bool = true;
//     let condition: bool = false;
//     let x: i32 = if condition {5} else {6};

//     println!("{x}")
// }



// fn main() {
//     // let condition: bool = true;
//     let condition: bool = false;

//     // let number = if condition { 5 } else { "six" };

//     if condition {
//         let number: i32 = 5;
//         println!("The value of number is: {number}");
//     } else {
//         let number: &str = "six";
//         println!("The value of number is: {number}");
//     }
// }





// fn main () {

//     let cond = true;
//     let x;
//     if cond {
//     x = 1;
//     } else {
//     x = 2;
//     }
//     println!("{x}")
// }


// fn main() {
//     loop {
//         println!("again!");
//     }
// }




// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 25;

//         if counter == 25 {
//             break counter * counter;
//         }
//     };

//     println!("The result is {result}");
// }




// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;
//         let mut multiply: i32 = 2;

//         'counting_down: loop {
//             println!("remaining = {remaining}");

//             'mult: loop {
//                 multiply = multiply * 2;
//                 if multiply == 1024 {
//                     // break 'counting_down
//                     break 'counting_up
//                 }
//             }
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }

//         count += 1;
//     }
//     println!("End count = {count}");
// }


// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{number}!");

//         number -= 1;
//         if number == 1 {
//             break
//         }
//     }

//     println!("LIFTOFF!!!");
// }

// use rand::Rng;
// fn main() {
//     // let a = [1, 2, 3, 4, 5];
//     let a: [i32; 100] = [rand::thread_rng().gen_range(1..=100); 100];
//     let mut index = 0;

//     while index < a.len() {
//         println!("the value at index {} is: {}",index, a[index]);

//         index += 1;
//     }
// }

// fn main() {
//     let array_of_numbers: [i32; 5] = [69, 228, 322, 666, 777];
//     for number in array_of_numbers {
//         println!("{number}")
//     }

// }



// fn main() {
//     for number in (1..4).rev() {
//         println!("{number}!");
//     }
//     println!("LIFTOFF!!!");
// }



// fn main() {
//     let mut x = 0;
//     'a: loop {
//         x += 1;
//         'b: loop {
//             if x > 10 {
//                 continue 'a;
//             } else {
//                 break 'b;
//             }      
//         }
//         break;       
//     }
// }




fn main() {
    let a = [5; 10];
    let mut sum = 0;
    for x in a {
        sum += x;
    }
    println!("{sum}");
}