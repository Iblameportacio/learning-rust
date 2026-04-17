//Objetivo: Haz un bucle del 1 al 20.
//Instrucción: Si el número es múltiplo de 3, imprime: "[número] es múltiplo de 3". Si no lo es, no imprimas nada.
//Pista: Recuerda el operador residuo %. Si i % 3 == 0, es múltiplo.
//Lo que practicas: Filtrado selectivo dentro de un bucle.
fn main() {
    for i in 1..=20 {
        if i % 3 == 0 {
            println!("{} es multiplo de 3", i)
        } else {
            //no hacer nada lmao
        }
    }
}
