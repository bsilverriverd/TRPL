enum IpAddrKind {
	V4,
	V6,
}

struct IpAddr1 {
	kind: IpAddrKind,
	address: String,
}

enum IpAddr2 {
	V4(String),
	V6(String),
}

enum IpAddr3 {
	V4(u8, u8, u8, u8),
	V6(String),
}

enum Message {
	Quit,
	Move { x: i32, y: i32},
	Write(String),
	ChangeColor(i32, i32, i32),
}

impl Message {
	fn call(&self) {

	}
}

fn main() {
	let four = IpAddrKind::V4 ;
	let six = IpAddrKind::V6 ;

	let home1 = IpAddr {
		kind: IpAddrKind::V4,
		address: String::from("127.0.0.1"),
	} ;
	let loopback2 = IpAddr {
		kind: IpAddrKind::V6,
		address: String::from("::1"),
	} ;

	let home2 = IpAddr2::V4(String::from("127.0.0.1")) ;
	let loopback2 = IpAddr2::V6(String::from("::1")) ;

	let home3 = IpAddr3::V4(127, 0, 0, 1) ;
	let loopback3 = IpAddr3::V6(String::from("::1")) ;

	struct QuitMessage ; // unit struct
	struct MoveMessage {
		x: i32,
		y: i32,
	}
	struct WriteMessage(String) ; // tuple struct
	struct ChangeColorMessage(i32, i32, i32) ;

	let m = Message::Write(String::from("hello")) ;
	m.call() ;

	let some_numver = Some(5) ;
	let some_string = Some("a string") ;

	let absent_number: Option<i32> = None ;

	let x : i8 = 5 ;
	let y = Option<i8> = Some(5) ;

	let sum x + y ; // compile error
}

fn route (ip_kind: IpAddrKind) {}
