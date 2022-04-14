//External Libraries
use rand::Rng;
//Standard Libraries
use std::io;
use std::cmp::Ordering;


fn main() {

    /*
        Generates a random number between 1 and 101
        "(...).gen_range(1,,=100)" does the same thing
    */
    let rand_number = rand::thread_rng().gen_range(1..101);

    //println!("O número gerado aleatoriamente é: {}", rand_number);

    loop {
        println!("Insira um número: ");

        let mut guess = String::new();

        // Same as " io::stdin().read_line(...).expect(...); "
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        /*
            trim() => removes all \n and \r of the string
            parse() => parse the string into a number
        */  
        //let guess: u32 = guess.trim().parse().expect("Favor inserir um número!");
        //If the user input is not a number, the program will contiue
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        

        //println!("O número inserido foi: {}", guess);

        match guess.cmp(&rand_number) {
            Ordering::Less => println!("Menor"),
            Ordering::Greater => println!("Maior"),
            Ordering::Equal => {
                println!("Acertou!!!");
                break;
            }

        }   
    }

    println!("Fim do programa\n");
}
