// use std::io;
// use rand::Rng;
// use std::cmp::Ordering;
// fn main(){
    //Listing 2-1: Code that gets a guess from the user and prints it
    // println!("Guess the number!");

    // println!("Please input your guess");

    // let mut guess: String = String::new();

    // io::stdin()
    //     .read_line(&mut guess)
    //     .expect("Faild to read line");

    // println!("You guessed number {guess}")




    //Listing 2-2: The output from running cargo build after adding the rand crate as a dependency
    // sypoo@sypoo-desktop:~/mtuci/summer_practise_2023/mtuci-rs$ cargo build
    // Compiling libc v0.2.147
    // Compiling cfg-if v1.0.0
    // Compiling ppv-lite86 v0.2.17
    // Compiling getrandom v0.2.10
    // Compiling rand_core v0.6.4
    // Compiling rand_chacha v0.3.1
    // Compiling rand v0.8.5
    // Compiling homework_1 v0.1.0 (/home/sypoo/mtuci/summer_practise_2023/mtuci-rs/homework_1)
    // Finished dev [unoptimized + debuginfo] target(s) in 1.31s



    // Listing 2-3: Adding code to generate a random number
    // println!("Guess the number!");

    // let secret_number: i32 = rand::thread_rng().gen_range(1..=100);


    // println!("The secret number is: {secret_number}");

    // println!("Please input your guess.");

    // let mut guess: String = String::new();

    // io::stdin()
    //     .read_line(&mut guess)
    //     .expect("Failed to read line");

    // println!("You guessed: {guess}");


    // Listing 2-4: Handling the possible return values of comparing two numbers
    // println!("Guess the number!");

    // let secret_number: u32 = rand::thread_rng().gen_range(1..=100);


    // println!("The secret number is: {secret_number}");

    // println!("Please input your guess.");

    // let mut guess: String = String::new();

    // io::stdin()
    //     .read_line(&mut guess)
    //     .expect("Failed to read line");

    // let guess: u32 = guess.trim().parse().expect("Please type a number!");

    // println!("You guessed: {guess}");

    // match guess.cmp(&secret_number){
    //     Ordering::Less => println!("Too small!"),
    //     Ordering::Greater => println!("Too big!"),
    //     Ordering::Equal => println!("You win!"),
    // }
        
    
    // Listing 2-5: Ignoring a non-number guess and asking for another guess instead of crashing the program
    // println!("Guess the number!");

    // let secret_number: u32 = rand::thread_rng().gen_range(1..=100);


    // println!("The secret number is: {secret_number}");

    // loop {

    //     println!("Please input your guess.");

    //     let mut guess: String = String::new();

    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read line");

    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };
    
    //     println!("You guessed: {guess}");

    //     match guess.cmp(&secret_number){
    //         Ordering::Less => println!("Too small!"),
    //         Ordering::Greater => println!("Too big!"),
    //         Ordering::Equal =>{
    //             println!("You win!");
    //             break;
    //         }
    //     }
    // }


// }


// Listing 2-6: Complete guessing game code
use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main(){

    println!("Guessing game");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        
        println!("Please input your guess!");
        
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");


        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {guess}!");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}