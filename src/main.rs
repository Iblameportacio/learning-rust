//Contador de Ceros: Cuenta cuántas veces aparece el número 0 en un array.
fn main() {
    let mut contador = 0;
    let v: Vec<i32> = vec![1, 0, 2, 3, 4, 5, 6, 0, 8, 9, 10, 0, 11];
    for x in &v {
        if *x == 0 {
            contador = contador + 1;
        }
    }
    println!(" hay {:?} ceros en el vector", contador)
}
