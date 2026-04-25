//Verificador de Rango: Función que diga si un número está entre 10 y 20 (inclusive).
fn main() {
    let uno: i32 = 1;
    let solucion: bool = esta(uno);
    println!(
        "el numero {:?} esta en el rango  entre 10 y 20? {:?}",
        uno, solucion
    );
}
fn esta(n: i32) -> bool {
    if n >= 10 && n <= 20 {
        return true;
    } else {
        return false;
    }
}
