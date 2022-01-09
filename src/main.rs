mod number_word;
mod simple_user_input;
use number_word::number_to_word;
use simple_user_input::get_numeric_input;


fn main() {
    println!("How much would you like to go up to?");
	let max:i32=get_numeric_input();
	fibonacci(max);
	
}
fn fibonacci(n:i32){
	let mut fib1:i32=0;
	let mut fib2:i32=1;
	let mut fib3:i32=1;
	while fib1<n {
		fib3=fib1+fib2;
		println!("{}", number_to_word(fib1));
		fib1=fib2;
		fib2=fib3;
	}
}