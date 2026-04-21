use std::vec;

//Buscador: Crea un vector y una función que diga si el número "10" está dentro.
fn main() {
    let v: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 8, 9, 10, 11];
    let resultado: bool = buscar(&v);
    println!("esta 10 en el vector {:?} ? {:?}", v, resultado);
}
fn buscar(n: &Vec<i32>) -> bool {
    n.contains(&10)
}
