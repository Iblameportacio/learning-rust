//El Reloj: Un bucle que imprima "Minuto X, Segundo Y" hasta llegar a 5 minutos.
fn main() {
    let segundos = 0;
    let minutos = 0;
    for i in minutos..=5 {
        for s in segundos..=59 {
            println!("minuto {} segundo {}", i, s)
        }
    }
}
