use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("
	┌─────────── •✧✧• ───────────┐
	    -Adivinhe o número !- 
	└─────────── •✧✧• ───────────┘
			");
    
    let numero_aleatorio= rand::thread_rng().gen_range(1..101);
    println!("---------------------------");
    // println!("O numero secreto é {}",numero_aleatorio);
    println!("Digite o número que você acha certo");
    
    
    loop{
    	let mut adivinhe = String::new();
	    io::stdin()
	    	.read_line(&mut adivinhe)
	    	.expect("Falhou a ler a linha");
	    let adivinhe: u32 = match adivinhe.trim().parse() {
	    	Ok(num) => num,
	    	Err(_) => continue, //O _ é um valor catchall, ele pega todos os valores possiveis de Err
	    };
	    println!("Você achou que era: {}", adivinhe);
	//Tipos inteiros em rust são padronizados por i32

	    match adivinhe.cmp(&numero_aleatorio){ //Nós fazemos a comparação usando cmp, ele vai retornar um Ordering que pode assumir 3 valores
	    										//Usamos a expressão match pra decidir qual o resultado que irá retornar

	    	Ordering::Less => println!("O valor é maior 👆 "),
	    	Ordering::Greater => println!("O valor é menor 👇"),
	    	Ordering::Equal => {
	   		println!("
	┌─────────── •✧✧• ───────────┐
	 -Parabéns, você ganhou !👊- 
	└─────────── •✧✧• ───────────┘
			");
	    		
	    		break;
	    	}
	    }
	}

//poderiamos tratar o "adivinhe" assim: let adivinhe: u32 = adivinhe.trim().parse().expect("Por favor, digite um numero")
// todo read_line retorna um Result, que é uma enumeração 
// dentro o Result pode ser Ok ou Err, 
	//Ok indica que a operação deu certo e dentro dele contém o valor de sucesso gerado 
	//Err indica que a operação deu errado e dentro dele contém como ou porque a operação falhou
//A proposta de Result é lidar com o tratamento de exceção
}
