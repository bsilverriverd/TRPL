fn main() {
	// Listing 8-11: Creating a new, empty String
	let mut s = String::new() ;

	// Listing 8-12: Using the to_string method to create a String from a string literal
	let data = "initial contents" ;

	let s = data.to_string() ;

	//the method also works on a literal directly
	let s = "initial contents".to_string() ;

	// Listing 8-13: Ising the String::from function to create a String from a string literal
	let s = String::from("initial contents") ;

	// Lisiting 8-15: Appending a string slice to a String using the push_str method
	let mut s = String::from("foo") ;
	s.push_str("bar") ;

	// Listing 8-16: Using a string slice after appengind its contents to a String
	let mut s1 = String::from("foo") ;
	let s2 = "bar" ;
	s1.push_str(s2) ;
	println!("s2 is {}", s2) ;
	
	// Listing 8-17: Adding one character to a String value using push
	let mut s = String::from("lo") ;
	s.push('l') ;

	// Listing 8-18: Using the + operator to combine two String values into a new String value
	let s1 = String::from("Hello, ") ;
	let s2 = String::from("worlds!") ;
	let s3 = s1 + &s2 ; // note s1 has been moved here and can no longer be used

	// Concatenating multiple strings usign the + operator
	let s1 = String::from("tic") ;
	let s2 = String::from("tac") ;
	let s3 = String::from("toe") ;

	let s = s1 + "-" + &s2 + "-" + &s3 ;

	// Using the format! macro for complicated string combination
	let s1 = String::from("tic") ;
	let s2 = String::from("tac") ;
	let s3 = String::from("toe") ;
	
	let s = format!("{}-{}-{}", s1, s2, s3) ;

/*
	// Listing 8-19: Attempting to use indexing syntax with a String
	let s1 = String::from("hello") ;
	let h = s1[0] ;
*/
	
}
