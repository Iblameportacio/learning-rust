//Suma de Vector: Crea un vector con 5 números y suma todos sus elementos.
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let suma = v[0] + v[1] + v[2] + v[3] + v[4];
    println!(
        "{:?} + {:?} + {:?} + {:?} + {:?} = {:?}",
        v[0], v[1], v[2], v[3], v[4], suma
    )
}
