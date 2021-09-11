fn fn_largest (list: &[i32]) -> i32 {
	let mut largest = list[0] ;

	for &item in list {
		if item > largest {
			largest = item ;
		}
	}

	largest
}

fn main() {
	// Listing 10-1: Code to find the largest number in a list of numbers
	let number_list = vec![34, 50, 25, 100, 65] ;

	let mut largest = number_list[0] ;

	for number in number_list {
		if number > largest {
			largest = number ;
		}
	}

	println!("The largest number is {}", largest) ;

	// Listing 10-2: Code to fing the largest number in two lists of numbers
	let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8] ;

	let mut largest = number_list[0] ;

	for number in number_list {
		if number > largest {
			largest = number ;
		}
	}

	println!("The largest number is {}", largest) ;

	// Listing 10-3: Abstracted code to find the largest number in two lists
	let number_list = vec![34, 50, 25, 100, 65] ;

	let result = fn_largest(&number_list) ;
	println!("The largest number is {}", result) ;

	let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8] ;

	let result = fn_largest(&number_list) ;
	println!("The largest number is {}", result) ;
}
