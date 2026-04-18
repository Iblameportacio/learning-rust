//Objetivo: Crear una función que reciba un número y diga si es primo o no.
//Instrucción: Crea fn es_primo(n: u32) -> bool. Dentro, usa un bucle que vaya de 2 hasta la mitad del número.
//Si algún número lo divide exactamente, devuelve false. Si termina el bucle sin encontrar divisores, devuelve true.
//En el main: Llama a la función con un número y usa un if para imprimir "Es primo" o "No es primo".
fn main() {
    let mi_numero = 13;
    let es_o_no: bool = es_primo(mi_numero);
    println!("el numero {} es primo {}", mi_numero, es_o_no)
}

fn es_primo(n: u32) -> bool {
    for i in 2..=n {
        if n % i == 0 {
            return false;
        } else if n % i != 0 {
            return true;
        }
    }
    true
}
