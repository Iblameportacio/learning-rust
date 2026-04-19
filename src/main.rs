//Contador de Impares: Cuenta cuántos números impares hay entre 1 y 100.
fn main() {
    let mut inicio = 0;
    for i in 1..=100 {
        if i % 2 != 0 {
            inicio = inicio + 1;
        }
    }
    println!(
        "el numero de pares de los numeros del 1 al 100 son\n {}",
        inicio
    )
}
//ya empiezo a notar que mi logica de prograacion ha mejorado mucho
