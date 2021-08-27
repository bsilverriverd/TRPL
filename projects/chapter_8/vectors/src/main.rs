fn main() {

	// Listing 8-1: Creating a new, empty vector to hold values of type i32
	let v: Vec<i32> = Vec::new() ;

	// Lissting 8-2: Creating a new vector containing values
	let v = vec![1, 2, 3] ;

	// Listing 8-3: Using the push method to add values to a vector
	let mut v = Vec::new() ;

	v.push(5) ;
	v.push(6) ;
	v.push(7) ;
	v.push(8) ;

	// Listing 8-4: Showing where the vector and its elements are dropped
	{
		let v = vec![1, 2, 3, 4] ;

		// do stuff with v
	} // v goes out of scope and its elements are dropped

	// Listind 8-5: Using indexing syntax or the get method to access an item in a vector
	let v = vec![1, 2, 3, 4, 5] ;

	let third: &i32 = &v[2] ;
	println!("The third element is {}", third) ;

	match v.get(2) {
		Some(third) => println!("The third element is {}", third),
		None => println!("There is no third element."),
	}
	
/*
	// Listing 8-6: Attempting to access the element at index 100 in a vector containing five elements
	let v = vec![1, 2, 3, 4, 5] ;

	let does_not_exist = &v[100] ;
	let does_not_exist = v.get(100) ;
*/
	
/*
	// Listing 8-7: Attempting to add an element to a vector while holding a reference to an item
	let mut v = vec![1, 2, 3, 4, 5] ;

	let first = &v[0] ;

	v.push(6) ;

	println!("The first element is: {}", first) ;
*/
	
	// Listing 8-8: Printing each element in a vector by iterating over the elements using a for loop
	let v = vec![100, 32, 57] ;
	for i in &v {
		println!("{}", i) ;
	}
	
	// Listing 8-9: Iterating over mutable refrences to elements in a vector
	let mut v = vec![100, 32, 57] ;
	for i in &mut v {
		*i += 50 ;
	}

	// Listing 8-10: Defining an enum to store values of different types in one vector
	enum SpreadsheetCell {
		Int(i32),
		Float(64),
		Txt(String),
	}

	let row = vec![
		SpreadsheetCell::Int(3),
		SpreadsheetCell::Text(String::from("blue")),
		SpreadsheetCell::Float(10.12),
	] ;

	
}
