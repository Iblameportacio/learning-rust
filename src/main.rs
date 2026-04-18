//Objetivo: En el kernel se mueven muchos datos. Vamos a intentar invertir una secuencia.
//Instrucción: Dado un array let datos = [1, 2, 3, 4, 5];,
//  usa un bucle for y el método .rev() para imprimir los números al revés, pero todos en la misma línea.
//Pista: Usa print! (sin el ln) para que no salte de línea, y un println!("") al final del bucle.
fn main() {
    let datos = [1, 2, 3, 4, 5];
    for d in datos.iter().rev() {
        print!("{:?}", d);
    }
}
