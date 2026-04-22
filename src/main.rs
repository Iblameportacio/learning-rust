//Fibonacci Simple: Imprime los primeros 10 números de la secuencia de Fibonacci.
// es igual a 0+1+1+2+3+4+5+6+7+8+9+10= 1,2,3,5,8,13,21...
fn main() {
    let mut a = 0;
    let mut b = 1;
    for _i in 0..10 {
        let i = a + b;
        let solucion = a;
        println!("{:?}", solucion);
        a = b;
        b = i;
    }
}
