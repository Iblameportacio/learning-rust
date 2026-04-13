//Ya sabemos que tu primo mide 2.16m. Vamos a hacer una lista de alturas y filtrar quiénes son "Gigantes".
//Tarea: Crea un vec![1.70, 1.90, 2.16, 1.65, 2.05].
//Lógica: Usa .iter() y .filter() para obtener solo las alturas mayores a 2.00m.
//Output esperado: "Los gigantes son: [2.16, 2.05]"
fn main() {
    let alturas: Vec<f32> = vec![1.70, 1.90, 2.16, 1.65, 2.05];
    let gigantes = alturas.iter().filter(|&h| h > &2.00).collect::<Vec<_>>();
    println!("Los gigantes son: {:?}", gigantes);
}
