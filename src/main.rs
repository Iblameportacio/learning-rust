//Simulador de Cajero: Tienes un saldo inicial, crea funciones para retirar y depositar.
fn main() {
    let saldo: f32 = 0.0;
    let deposito: f32 = 40.0;
    let retiro: f32 = 32.4;

    println!("tu saldo actual es {:?}", saldo);

    let saldo_despues_deposito = depositar(saldo, deposito);
    let saldo_total = retirar(saldo_despues_deposito, retiro);

    println!("depositaste {:?}", deposito);
    println!("retiraste {:?}", retiro);
    println!("tu saldo total disponible es {:?}", saldo_total);
}

fn depositar(n: f32, cantidad: f32) -> f32 {
    n + cantidad
}

fn retirar(n: f32, cantidad: f32) -> f32 {
    n - cantidad
}
