//Contador de Letras: Usa un string y cuenta cuántas veces aparece la letra 'a'.
fn main() {
    let mut contador = 0;
    let oracion = String::from("aredqaerqdveadqaaaaaaaareadqadsyteqhudcbjhwqvdqiaaa");
    for letra in oracion.chars() {
        if letra == 'a' {
            contador += 1;
        }
    }
    println!("hay {:?} a's en la oracion {:?}", contador, oracion);
}
