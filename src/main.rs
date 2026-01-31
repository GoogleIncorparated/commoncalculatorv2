enum Oper {
	plus(i32, i32),
	minus(i32, i32),
	stillidk(i32, i32),
	multiple(i32, i32),
}

fn test(s: &i32, f: &i32) -> Result<i32, String> {
	match f {
		0 => return Err(String::from("no")),
		_ => return Ok(s / f),
	};
}

impl Oper {
	fn calc(&self) {
	match self {
		Oper::plus(a, b) => println!("{}", a + b),
		Oper::minus(a,b) => println!("{}", a - b),
		Oper::stillidk(a,b) => match test(a,b) {
			Ok(number) => println!("{}", number),
			Err(what) => println!("{}", what),
		},
		Oper::multiple(a,b) => println!("{}", a * b),
	}
	}
}



fn main() {
let mut queue = Vec::new();
queue.push(Oper::plus(10,20));
queue.push(Oper::minus(100, 50));
queue.push(Oper::multiple(5,5));
queue.push(Oper::stillidk(5,0));

for opr in &queue {
	opr.calc();
}

}
