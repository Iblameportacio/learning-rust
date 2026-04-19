//Multiplicador Variable: Pide un número y muestra su tabla del 1 al 12.
use std::io;

fn main() {
    println!("escoje un numero para mostrar su tabla de multi\n");
    let mut entrada = String::new();

    io::stdin().read_line(&mut entrada).unwrap();

    let numero: i32 = entrada.trim().parse().unwrap();

    println!("la tabla de multiplicar es:\n");

    for i in 1..=12 {
        println!("{} x {} = {}", numero, i, numero * i);
    }
}
