///Tarea: Crea un vec!["ls", "cd", "sudo", "pacman"].
//Lógica: Usa .iter(), .map() y el método .to_uppercase() para convertir todos los comandos a mayúsculas.
//Output esperado: ["LS", "CD", "SUDO", "PACMAN"] guardado en un nuevo Vec<String>.
fn main() {
    let comandos = vec!["ls", "cd", "sudo", "pacman"];
    let comandos_mayusculas: Vec<String> = comandos.iter().map(|c| c.to_uppercase()).collect();
    println!("{:?}", comandos_mayusculas);
}
