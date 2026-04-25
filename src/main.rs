//Password Checker: Función que reciba un string y diga si tiene más de 8 caracteres.
fn main() {
    let mut contador = 0;
    let password = String::from("aredqaerqd2819ywqyg73");
    for _i in password.chars() {
        contador += 1;
    }
    println!("hay {:?} caracteres en la oracion {:?}", contador, password);
}
