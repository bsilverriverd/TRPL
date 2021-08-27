use std::collections::HashMap ;

fn main() {
	let mut v = vec![10, 1, 8, 3, 6, 9, 4, 7, 2, 5, 10] ;

	v.sort() ;

	let mut sum: i32 = 0 ;
	
	for i in &v {
		sum += i ;	
	}

	let mean: f32 = sum as f32 / v.len() as f32 ;
	
	println!("mean: {}", mean) ;

	let median: &i32 = &v[(v.len() - 1) / 2] ;
	
	println!("median: {}", median) ;

	let mut map = HashMap::new() ;

	for i in &v {
		let count = map.entry(i).or_insert(0) ;
		*count += 1 ;
	}

	let mut mode = None ;
	let mut max: i32 = 0 ;

	for (key, count) in map.iter() {
		if *count > max {
			max = *count ;
			mode = Some(*key) ;
		}
	}
	match mode {
		Some(_) => println!("mode: {:?}", mode) ,
		None => println!("No mode") ,
	}
}
