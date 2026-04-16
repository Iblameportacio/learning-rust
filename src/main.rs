//crea un bucl con numeros del 1 al 10, si es mayor a 10 es grande sino es pequeño
fn main() {
    for i in 1..=15 {
        if i > 10 {
            println!("el numero {} es grande", i)
        } else {
            println!("el numero {} es pequeño", i)
        }
    }
}
