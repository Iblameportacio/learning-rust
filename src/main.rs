//Suma de Rango: Crea una función que reciba dos números y sume todo lo que hay entre ellos.
fn main() {
    let resultado = suma_rango(1, 10); // Aquí eliges tú los números
    println!("La suma del rango es: {}", resultado);
}

fn suma_rango(inicio: i32, fin: i32) -> i32 {
    let mut acumulador = 0;
    for i in inicio..=fin {
        if i >= 1 {
            acumulador = acumulador + i
        }
    }
    // y ve sumando cada número al acumulador.

    acumulador // Al final devuelves el total
}
//niceee
