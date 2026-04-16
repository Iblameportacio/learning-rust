//Objetivo: Imprimir la tabla del 7, desde $7 \times 1$ hasta $7 \times 10$.Lo que usas:
// Un bucle for que vaya del 1 al 10 y una multiplicación simple.
// Resultado esperado: Deberías ver en consola: 7 x 1 = 7, 7 x 2 = 14, etc.
fn main() {
    let tabla_del = 7; // Tu variable fija

    for i in 1..=10 {
        // El for crea la variable 'i' que va cambiando
        let resultado = tabla_del * i; // Aquí sacas el resultado en cada vuelta
        println!("{} x {} = {}", tabla_del, i, resultado);
    }
}
