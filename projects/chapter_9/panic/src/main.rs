use std::fs::File ;
use std::io::ErrorKind ;

use std::io ;
use std::io::Read ;

// Listing 9-6: A function that returns errors to the calling code using match
fn read_username_from_file() -> Result<String, io::Error> {
	let f = File::open("hello.txt")  ;
	
	let mut f = match f {
		Ok(file) => file,
		Err(e) => return Err(e),
	} ;

	let mut s = String::new() ;

	match f.read_to_string(&mut s) {
		Ok(_) => Ok(s),
		Err(e) => Err(e),
	}
}

// Listing 9-7: A function that returns errors to the calling code using the ? operator
fn read_username_from_file() -> Result<String, io:Error> {
	let mut f = File::open("hello.txt")?;
	let mut s = String::new() ;
	f.read_to_string(&mut s)? ;
	Ok(s)
}

// Listing 9-8: Chaining method calls after the ? operator
fn read_username_from_file() -> Result<String, io::Error> {
	let mut s = String::new() ;
	File::open("hello.txt")?.read_to_string(&mut s)? ;
	Ok(s)
}

// Listing 9-9: using fs::read_to_string instead of opening and then reading the file
fn read_username_from_file() -> Result<String, io::Error> {
	fs::read_to_string("hello.txt")
}

fn main() {
//	panic!("crash and burn") ;

	// Listting 9-1: Attepting to access an element beyond the end of a vector, which will cause a call to panic!
/*	
	let v = vec![1, 2, 3] ;
	v[99] ;
*/
	//Listing 9-3: Opening a file
	let f = File::open("hello.txt") ;

	// Listing 9-4: Using a match expression to handle the result variants that might be returned
/*
	let f = match f {
		Ok(file) => file,
		Err(error) => panic!("Problem opening the file: {:?}", error) ,
	} ;
*/
/*
	// Listing 9-5: Handling different kinds of errors in different ways
	let f match f {
		Ok(file) => file,
		Err(error) => match error.kind() {
			ErrorKind::NotFound => match File::create("hello.txt") {
				Ok(fc) => fc,
				Err(e) => panic!("Problem creating the file: {:?}", e),
			},
			other_error => {
				panic!("Problem opeing the file: {:?}", other_error)
			}
		},
	};
*/
	let f = File::open("hello.txt").unwrap() ; // call panic! macro if Result value is Err variant
	let f = File::open("hello.txt").ecpect("Failed to open hello.txt"); // write own error message
}
