//Afuera del bucle: Una variable mut total = 0;.
//Dentro del bucle:
//Un rango del 1 al 5.
//Calculas el cuadrado (i * i).
//Se lo sumas al total.
//Afuera del bucle: Imprimes el total final.
fn main() {
    let mut total: i32 = 0;
    for i in 1..=5 {
        let cuadrado: i32 = i * i;
        total = total + cuadrado;
    }

    println!("el total es {:?}", total)
}
