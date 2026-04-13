//Como te gusta el negocio de las laptops y Temu, vamos a automatizar el cálculo de ganancias.
//Tarea: Crea tres variables: costo (f32), precio_venta (f32) y nombre_articulo.
//Lógica: Calcula la ganancia (venta - costo) y el porcentaje de margen.
//Output esperado: "Articulo: ThinkPad | nancia: $350.000 | Margen: 77%"
fn main() {
    let costo: f32 = 1000000.0;
    let precio_venta: f32 = 1350000.0;
    let _nombre_articulo: &str = "ThinkPad";
    let ganancia = precio_venta - costo;
    let _margen = (ganancia / precio_venta) * 100.0;
    println!(
        "Articulo: {} | Ganancia: {} | Margen: {}%",
        _nombre_articulo, ganancia, _margen
    )
}
