//Calculadora de IVA: Función que reciba un precio y le sume el 19% (IVA de Colombia).
fn main() {
    let compra: f64 = 40.000;
    let total = iva(compra);
    println!(
        "la compra de {:?} termina siendo {:?} con un 19% de iva",
        compra, total
    );
}
fn iva(n: f64) -> f64 {
    let n: f64 = (n * 0.19) + n;
    n
}
