//calula el prdocuto de 6!
fn main() {
    let mut producto: i16 = 1;
    for i in (1..=6).rev() {
        producto *= i;
    }
    println!("el resultado es {}", producto)
}
