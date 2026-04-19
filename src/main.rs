//Es Par?: Función que devuelva bool si un número es par.
fn main() {
    let cinco: f32 = 5.0;
    let resultado = espar(cinco);
    println!("este numero {} es par?{}", cinco, resultado);
}
fn espar(n: f32) -> bool {
    if n % 2.0 == 0.0 {
        return true;
    } else {
        return false;
    }
}
