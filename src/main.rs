fn main()
{
    let mut minha_string: String = String::from("Olá, meu nome é Fernando. ");
    println!("Tamanho: {}", minha_string.len());
    println!("String vazia?: {}", minha_string.is_empty());
    for parte_do_texto in minha_string.split_whitespace()
    {
        println!("{}", parte_do_texto);
    }
    println!("Contem Fernando? {}", minha_string.contains("Fernando"));
    minha_string.push_str("Bem vindo a aula!");
    println!("{}", minha_string);
}
