//Palíndromo: Función que diga si una palabra se lee igual al revés (ej: "ana").
fn main() {
    let sentence = String::from("oso"); //we already know this is a palindrome
    let solved: bool = palindrome(sentence.clone());
    println!("the sentence {:?} is a palindrome? {:?}", sentence, solved);
}
fn palindrome(s: String) -> bool {
    let z = s.chars().rev().collect::<String>();
    z == s
}
