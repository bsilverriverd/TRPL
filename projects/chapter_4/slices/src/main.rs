fn main() {
	let s = String::from("hello world") ;

	let word = first_word(&s) ;

	//s.clear() ; // compile error

	println!("{}", word) ;

	let my_string = String::from("bye beauty") ;

	// first_word_im works on slices of 'String's
	let my_word1 = first_word_im(&my_string[..]) ;

	let my_string_literal = "bye beauty" ;

	// first_word_im works on slices of string literals
	let my_word2 = first_word_im(&my_string_literal[..]) ;

	// Because string literals *are* string slices already
	let my_word3 = first_word_im(&my_string_literal) ;

	println!("{}, {}, {}", my_word1, my_word2, my_word3) ;
}

fn first_word (s: &String) -> &str {
	let bytes = s.as_bytes() ;

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[0..i] ;
		}
	}
	
	&s[..]
}

fn first_word_im (s: &str) -> &str {
	let bytes = s.as_bytes() ;

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[0..i] ;
		}
	}
	
	&s[..]
}
