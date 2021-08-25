#[derive(Debug)]

struct Rectangle {
	width: u32,
	height: u32,
}

impl Rectangle {
	fn area(&self) -> u32 {
		self.width * self.height
	}

	fn can_hold(&self, other: &Rectangle) -> bool {
		self.width > other.width && self.height > other.height
	}

	fn square(size: u32) -> Rectangle {
		Rectangle {
			width: size,
			height: size,
		}
	}
}

fn main() {
	
	// Listing 5-8: Calculating the area of a rectangle specified by seperate width and height variables
	let width1 = 30 ;
	let height1 = 50 ;

	println!(
		"The area of the rectangle is {} square pixels.",
		area(width1, height1)
	) ;

	// Listing 5-9: Specifiying the width and height of the rectangle with a tuple
	let rect1 = (30, 50) ;

	println!(
		"The area of the rectangle is {} square pixels.",
		area2(rect1)
	) ;

	// Listing 5-10: Defining a Rectagle struct
	let rect1 = Rectangle {
		width: 30,
		height: 50,
	} ;

	println!(
		"The area of the rectangle is {} square pixels.",
		area3(&rect1)
	) ;
	
	// Listing 5-12: Adding the annotation to derive the Debug trait and printing the Rectangle instance using debug formatting
	println!("rect1 is {:?}", rect1) ;

	// Listing 5-13: Defining an area method on the Rectangle struct
	println!(
		"The area of the rectangle is {} square pixels.",
		rect1.area()
	) ;
	
	// Listing 5-14: Using the as-yet-unwritten can_hold method
	let rect2 = Rectangle {
		width: 10,
		height: 40,
	} ;
	let rect3 = Rectangle {
		width: 60,
		height: 45,
	} ;

	println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2)) ;
	println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3)) ;

	// Associated functions
	let sq = Rectangle::square(3) ;
	println!("sq is {:?}", sq) ;
	println!(
		"The area of sq is {} square pixels.",
		sq.area()
	) ;
}

fn area(width: u32, height: u32) -> u32 {
	width * height	
}

fn area2 (dimensions: (u32, u32)) -> u32 {
	dimensions.0 * dimensions.1
}

fn area3 (rectangle: &Rectangle) -> u32 {
	rectangle.width * rectangle.height
}
