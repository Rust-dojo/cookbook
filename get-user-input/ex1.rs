// Entrada de texto por uma pessoa
//
// Como pedir uma informação para a usuária, enquanto o programa está rodando.
fn main() {
    println!("Qual o seu nome? ");
    let mut name = String::new();
    let name_input_result = std::io::stdin().read_line(&mut name);
    match name_input_result {
        Ok(_size) => println!("Olá, {}!", name.trim()),
        Err(error) => println!("Ocorreu um erro: {}", error),
    }
}
