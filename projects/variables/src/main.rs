use std::io ;

fn main() {
	let mut x = 5 ;
	println!("The value of x is: {}", x) ;
	x = 6 ;
	println!("The value of x is: {}", x) ;
	
	// shadowing

	let y = 5 ;

	let y = y + 1 ;
	 
	let y = y * 2 ;

	println!("The value of y is: {}", y) ;
	
	// tuples
	let tup: (i32, f64, u8) = (500, 6.4, 1) ; // let tup = (500, 6.4, 1) ;

	let (a, b, c) = tup ; // destructing

	println!("The value of b is: {}", b) ;

	let five_hundred = tup.0 ;

	let six_point_four = tup.1 ;

	let one = tup.2 ;

	// arrays
	let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"] ;

	let arr: [i32; 5] = [1, 2, 3, 4, 5] ;

	let arr2 = [3; 5] ; // let arr2 = [3, 3, 3, 3, 3] ;

	println!("Please enter an array index.") ;

	let mut index = String::new() ;

	io::stdin()
		.read_line(&mut index)
		.expect("Failed to read line") ;

	let index: usize = index
		.trim()
		.parse()
		.expect("Index entered was not a number") ;

	let element = arr[index] ;

	println!(
		"The value of the element at index {} is: {}",
		index, element
	) ;

	// functions
	another_function(5) ;

	//statements and expressions

	let x1 = 5 ; // statement
	let x2 = { // expression
		let x = 3 ;
		x + 1
	} ;

	println!("The value of x2 is: {}", x2) ;

	// functions with return values
	let ret = five() ;
	println!("The value of ret is: {}", ret) ;

	let ret2 = plus_one(5) ;
	println!("The value of ret2 is: {}", ret2) ;
}

fn another_function(x: i32) {
	println!("The value of x is {}", x) ;
}

fn five() -> i32 {
	5
}

fn plus_one(x: i32) -> i32 {
	x + 1
}
