fn main() {
	
	// borrowing
	let s1 = String::from("hello") ;
	
	let len = calculate_length(&s1) ;

	println!("The Length of '{}' is {}.", s1, len) ;

	// mutable references
	let mut s = String::from("hello") ;

	change(&mut s) ;

	println!("{}", s) ;

	/* cannot have multiple mutable references to a particular piece of data in a particular scope
	let r1 = &mut s ;
	let r2 = &mut s ;
	this prevents data races */

	/* cannot have a mutable reference while there is a immutable one
	let r1 = &s ; // no problem
	let r2 = &s ; // no problem
	let r3 = &mut s ; // problem
	println!("{}, {}, {}", r1, r2, r3) ;
	*/

	let r1 = &s ; // no problem
	let r2 = &s ; // no problem
	println!("{}, {}", r1, r2) ;
	// r1 and r2 no longer used after this point

	let r3 = &mut s ; //no problem
	println!("{}", r3) ;

	// dangling references
	/* compile error
	let reference_to_nothing = dangle() ;
	*/
}

fn calculate_length (s: &String) -> usize { // s is a reference to a String
	s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
	// it refers to, nothing happens.

fn change (some_string: &mut String) {
	some_string.push_str(", world") ;
}

fn dangle () -> &String {
	let s = String::from("hello") ;

	&s
}
