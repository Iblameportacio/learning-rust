//Promedio de Notas: Función que reciba 3 notas ($f32$) y diga si aprueba (mayor a 3.0).
fn main() {
    let nota1: f64 = 4.0;
    let nota2: f64 = 1.0;
    let nota3: f64 = 3.0;
    let notas: (f64, f64, f64) = (nota1, nota2, nota3);
    let solucion: f64 = promedio(notas);
    println!(" el promedio de las notas {:?} es {:?}", notas, solucion);
}
fn promedio(nums: (f64, f64, f64)) -> f64 {
    let nums = (nums.0 + nums.1 + nums.2) / 3.0;
    nums
}
