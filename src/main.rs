//Conversor de C/F: Función que pase de grados Celsius a Fahrenheit.
fn main() {
    let gradoc: f32 = 0.0;
    let resultado = fahrenheit(gradoc);
    println!("los grados {} a fahrenheit son {}", gradoc, resultado);
}
fn fahrenheit(n: f32) -> f32 {
    let n = (n * 1.8) + 32.0;
    n
}
