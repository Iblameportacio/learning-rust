//El 7 de la suerte: Imprime los números del 1 al 50, pero si es múltiplo de 7, imprime "¡SUERTE!".
fn main() {
    for i in 1..=50 {
        println!("{}", i);
        if i % 7 == 0 {
            println!("¡SUERTE!");
        }
    }
}
