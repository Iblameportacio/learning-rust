//Objetivo: Dado el número 24, cuenta cuántos divisores exactos tiene
// (números que lo dividen y dejan residuo 0).
//Instrucción: 1.  Crea una variable fija let numero = 24;.
//2.  Crea un acumulador mut contador = 0;.
//3.  Haz un bucle del 1 al 24.
//4.  Si numero % i == 0, súmale 1 al contador.
//Resultado esperado: "El número 24 tiene [X] divisores"
fn main() {
    let numero: u8 = 24;
    let mut contador = 0;
    for i in 1..=24 {
        if numero % i == 0 {
            contador = contador + 1
        }
    }
    println!("el numero 24 tiene {} divisores", contador)
}
