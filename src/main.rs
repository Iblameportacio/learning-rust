use std::vec;
//Inversor Manual: Crea un vector y pásalo a otro vector pero con los elementos al revés.
fn main() {
    let v: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 8, 9, 10, 11];
    let resultado: Vec<i32> = v.iter().rev().cloned().collect();
    println!("el vector {:?} invertido es {:?}", v, resultado);
}
