//El Buscador de Promedios (Tipos de datos):
//Objetivo: En el kernel, los decimales son peligrosos. Vamos a practicar con f32.
//Instrucción: Suma los números del 1 al 10 y calcula el promedio.
//Pista: Para que el promedio sea exacto, tendrás que convertir la suma a decimal:
// let promedio = suma as f32 / 10.0;.
fn main() {
    let mut suma = 0;
    for i in 1..=10 {
        suma += i;
    }
    let promedio = suma as f32 / 10.0;
    println!("el promedio es {}", promedio)
}
