enum Oper {
	plus(i32, i32),
	minus(i32, i32),
	stillidk(i32, i32),
	multiple(i32, i32),
}



impl Oper {
	fn calc(&self) {
	match self {
		Oper::plus(a, b) => println!("{}", a + b),
		Oper::minus(a,b) => println!("{}", a - b),
		Oper::stillidk(a,b) => println!("{}", a / b),
		Oper::multiple(a,b) => println!("{}", a * b),
	}
	}
}



fn main() {
let mut queue = Vec::new();
queue.push(Oper::plus(10,20));
queue.push(Oper::minus(100, 50));
queue.push(Oper::multiple(5,5));

for opr in &queue {
	opr.calc();
}

}
