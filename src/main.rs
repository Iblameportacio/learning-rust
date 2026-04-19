//Suma de Pares: Suma todos los números pares del 1 al 50.
fn main() {
    let mut inicio = 0;
    for i in 1..=50 {
        inicio = inicio + i;
    }
    println!("{}", inicio)
}
