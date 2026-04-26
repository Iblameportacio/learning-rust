//Mínimo Común Múltiplo: Crea una función para hallarlo entre dos números.(el numero mas pequeño
// que es multiplo de ambos)
fn main() {
    let a: i32 = 45;
    let b: i32 = 18;
    let solucion: i32 = mcm(a, b);
    println!("el mcm de los numeros {:?} y {:?} es {:?}", a, b, solucion);
}

fn mcm(n: i32, m: i32) -> i32 {
    let mut candidato;
    if n > m {
        candidato = n;
    } else {
        candidato = m;
    }

    loop {
        if candidato % n == 0 && candidato % m == 0 {
            return candidato;
        } else {
            candidato += 1;
        }
    }
}
//diferencia clave en un loop break sale del loop
// return sale de la funcion
