//Máximo de Tres: Función que reciba 3 números y devuelva el mayor de ellos.
fn main() {
    let numeros: (i32, i32, i32) = (1, 432, 873);
    let resultado = elmayor(numeros);
    println!("en {:?} el mayor es {:?}", numeros, resultado);
}
fn elmayor(nums: (i32, i32, i32)) -> i32 {
    if nums.0 > nums.1 && nums.0 > nums.2 {
        return nums.0;
    } else if nums.1 > nums.0 && nums.1 > nums.2 {
        return nums.1;
    } else {
        return nums.2;
    }
}
