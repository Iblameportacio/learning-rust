use std::vec;
//Filtro de Positivos: Dado un vector con números negativos y positivos,
// crea uno nuevo solo con los positivos.
fn main() {
    let v: Vec<i32> = vec![1, 2, 3, 4, -5, 6, 8, -9];
    let ve: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 8, 9];
    println!("el vector v es {:?} y el vector ve es {:?}", v, ve);
}
