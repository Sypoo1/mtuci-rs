// Напишите программу, которая выводит на экран числа от 1 до 100. 
//При этом вместо чисел, кратных трем, программа должна выводить слово Fizz,
// а вместо чисел, кратных пяти — слово Buzz. Если число кратно пятнадцати,
//  то программа должна выводить слово FizzBuzz. 

fn main() {

    for number in 1..=100  {
        
        if number % 15 == 0 {
            println!("FizzBuzz")
        }

        else if number % 3 == 0 {
            println!("Fizz")
        }

        else if number % 5 == 0 {
            println!("Buzz")
        }
        
        else {
            println!("{}", number)
        }
    }
}