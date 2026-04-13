//En lugar de variables sueltas, vamos a agrupar datos.
//Tarea: Crea una struct llamada Laptop con los campos: modelo (String), ram (u32) y precio (f32).
//Lógica: Instancia una laptop (ej. una T470), cámbiale el precio (necesitarás let mut) e imprímela usando {:?}.
//Tip: Recuerda añadir #[derive(Debug)] arriba de la struct para poder imprimirla.
#[derive(Debug)]
struct Laptop {
    modelo: String,
    ram: u32,
    precio: f32,
}

fn main() {
    let mut t470 = Laptop {
        modelo: String::from("T470"),
        ram: 16,
        precio: 600.0_f32,
    };
    t470.precio = 550.0;
    println!("{}", t470.modelo);
    println!("{}", t470.ram);
    println!("{}", t470.precio);
}
