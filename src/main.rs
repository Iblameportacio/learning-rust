//Objetivo: Imprime los números del 10 al 1 (en orden descendente) y al final imprime "¡IGNICIÓN!".
fn main() {
    for i in (1..=10).rev() {
        println!("explosion en {}", i);
    }

    println!("¡IGNICIÓN!");
}
