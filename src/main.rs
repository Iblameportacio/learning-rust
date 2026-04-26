//FizzBuzz Extreme: Del 1 al 100, múltiplos de 3 (Fizz), de 5 (Buzz), de ambos (FizzBuzz) y de 7 (Bazz).
fn main() {
    for i in 1..=100 {
        if i % 7 == 0 && i % 5 == 0 && i % 3 == 0 {
            println!("{} Bazz", i)
        } else if i % 3 == 0 && i % 5 == 0 {
            println!("{} FizzBuzz", i)
        } else if i % 3 == 0 {
            println!("{} Fizz", i)
        } else if i % 5 == 0 {
            println!("{} Buzz", i)
        }
    }
}
