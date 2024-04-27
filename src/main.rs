fn main()
{
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

        // &str
        let nome_a = "Hello";

        // std::String
        let nome_b = String::from("Hello world");
    
        //&str
        let hello = &nome_b[0..5];
        let world = &nome_b[6..11];
    
        println!("conteúdo da variável hello: >>{}<<", hello);
        println!("conteúdo da variável nome_a: >>{}<<", nome_a);
        println!("conteúdo da variável world: >>{}<<", world);
}
