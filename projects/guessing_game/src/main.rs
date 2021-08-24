// Listing 2-3: Adding code to generate a random number
// Listing 2-4: Handling the possible return values of comparing two numbers
// Listing 2-5: Ignoring a non-number guess and asking for another guess instead of crashing the program
use std::io ;
use std::cmp::Ordering ;
use rand::Rng ;

fn main() {
	println!("Guess the number!") ;

	let secret_number = rand::thread_rng().gen_range(1..101) ; // (1..=100)
	
	loop {
		println!("Please input your guess.") ;
	
		let mut guess = String::new() ;
	
		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read line") ;
	
		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		} ;
	
		println!("You guessed: {}", guess) ;
	
		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
				println!("You win!") ;
				break ;
			}
		}
	}
}
