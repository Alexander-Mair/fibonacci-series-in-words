
use text_io;
use std::io;
    pub fn get_input(prompt: &str) -> String{
        println!("{}",prompt);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {},
            Err(_no_updates_is_fine) => {},
        }
        input.trim().to_string()
    }
	pub fn get_numeric_input() -> i32{
	let i: i32 = text_io::read!();
   
	return i;
	}
