//hacer una escalera de asteriscos
fn main() {
    for i in 1..=5 {
        let resultado: String = "*".repeat(i);
        println!("{}", resultado)
    }
}
