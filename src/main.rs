//Objetivo: Encontrar el número más grande dentro de una secuencia (aunque la hagamos manual).
//Instrucción:
//Imagina que tienes estos números: [10, 55, 32, 5, 90, 12].
//Crea una variable mut mayor = 0;.
//Haz un bucle que recorra esos números (puedes usar un array si te atreves: for n in [10, 55, 32, 5, 90, 12]).
//Si el número actual n es mayor que mayor, entonces mayor ahora vale n.
//Resultado esperado: "El número más grande es 90".
fn main() {
    let numeros: [i32; 6] = [10, 55, 32, 5, 90, 12];
    let mut mayor: i32 = 0;
    for n in numeros.iter().map(|&n| n) {
        if n > mayor {
            mayor = n
        }
    }
    println!("el numero mas grande es {}", mayor)
}
