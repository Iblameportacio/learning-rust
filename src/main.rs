//Potencias de 2: Imprime las potencias de 2 ($2^1, 2^2, ...$) hasta llegar a $2^{10}$.
fn main() {
    let dos: i32 = 2;
    for i in 1..=10 {
        println!("{} ^ {} = {}", dos, i, 2_i32.pow(i));
    }
}
//no bro me siento como si hubiera tomado nzt lmaoo
