//Vamos a convertir notas numéricas a conceptos usando el match que tanto nos sirve.
//Tarea: Crea una variable nota (f32).
//Lógica: Usa un match (puedes convertir la nota a entero con as i32 si quieres usar rangos como 0..=2).
//3.0 a 3.9 -> "Pasaste raspando"
//4.0 a 5.0 -> "bien ahi bro"
//Menos de 3.0 -> "F por ti, broski"
//Output: El mensaje correspondiente.
fn main() {
    let nota: f32 = 4.5;
    match nota {
        3.0..=3.9 => println!("Pasaste raspando"),
        4.0..=5.0 => println!("bien ahi bro"),
        _ => println!("F por ti, broski"),
    }
}
