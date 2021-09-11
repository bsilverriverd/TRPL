fn largest_i32 (list: &[i32]) -> i32 {
	let mut largest = list[0] ;
	
	for &item in list {
		if item > largest {
			largest = item ;
		}
	}

	largest
}

fn largest_char (list: &[char]) -> char {
	let mut largest = list[0] ;

	for &item in list {
		if item > largest {
			largest = item ;
		}
	}

	largest 
}

fn largest<T: PartialOrd + Copy> (list: &[T]) -> T {
	let mut largest = list[0] ;

	for &item in list {
		if item > largest {
			largest = item ;
		}
	}

	largest
}

struct Point<T> {
	x: T,
	y: T,
}

impl<T>  Point<T> {
	fn x(&self) -> &T {
		&self.x
	}
}
fn main() {

	// Listing 10-4: Two functions that differ only in their names and the types in their signatures
	let number_list = vec![34, 50, 25, 100, 65] ;

	let result = largest_i32(&number_list) ;
	println!("The largest number is {}", result) ;

	let char_list = vec!['y', 'm', 'a', 'q'] ;

	let result = largest_char(&char_list) ;
	println!("The largest char is {}", result) ;

	// Listing 10-5: A definition of the largest function that uses generic type parameters but doesn't compile yet
/*
	let result = largest(&number_list) ;
	println!("The largest number is {}", result) ;

	let result = largest(&char_list) ;
	println!("The largest char is {}", result) ;
*/	
	// Listing 10-6: A Point<T> struct that holds x and y values of type T
	let integer = Point { x: 5, y: 10 } ;
	let float = Point { x: 1.0, y: 4.0 } ;
	
	// Listing 10-9: Implementing a method names x on the Point<T> struct that will return a reference to the x field  of type T
	let p = Point { x: 5, y: 10 } ;

	println!("p.x = {}", p.x()) ;
}
