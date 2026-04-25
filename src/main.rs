//Suma de Dígitos: (Difícil) Toma el número 123 y suma $1 + 2 + 3 = 6$.
fn main() {
    let mut n = 123;
    println!("la suma de los numeros {:?} es igual a ", n);
    let mut suma = 0;
    while n > 0 {
        suma += n % 10;
        n /= 10;
    }
    println!("{}", suma)
}
