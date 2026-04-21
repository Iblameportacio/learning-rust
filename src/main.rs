use std::vec;
//Duplicador: Multiplica por 2 cada elemento de un vector.
fn main() {
    let v: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 8, 9, 10, 11];
    let resultado: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("el vector {:?} multiplicado por dos es {:?}", v, resultado);
}
