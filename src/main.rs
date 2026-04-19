use std::f64::consts::PI;

//Área de Círculo: Función que recibe el radio ($f32$) y devuelve el área.
fn main() {
    let radio: f32 = 32.0;

    let resultado = area(radio);
    println!("el ares es {} cm²", resultado);
}
fn area(n: f32) -> f64 {
    let n = n as f64;
    PI * n * n
}
