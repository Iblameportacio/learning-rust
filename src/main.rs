//Verificador de Rango: Función que diga si un número está entre 10 y 20 (inclusive).
fn main() {
    let mut inicio = 1;
    for i in 1..=10 {
        inicio *= i;
    }
    println!("el factorial de 10 es {:?}", inicio)
}
