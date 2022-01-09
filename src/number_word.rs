pub fn number_to_word(mut n:i32)->String{
	let mut word=String::new();
	if n >999 && n<10000 {
		let m=(n/1000)*1000;
		match m {
	1000=>word=String::from("one thousand "),
	2000=>word=String::from("two thousand "),
	3000=>word=String::from("three thousand "),
	4000=>word=String::from("four thousand "),
	5000=>word=String::from("five thousand "),
	6000=>word=String::from("six thousand "),
	7000=>word=String::from("seven thousand "),
	8000=>word=String::from("eight thousand "),
	9000=>word=String::from("nine thousand "),
	_=>()
	}
	
	n=n%1000;
	}
	if n >99 && n<1000 {
		let m=(n/100)*100;
		match m {
	100=>word.push_str("one hundred"),
	200=>word.push_str("two hundred"),
	300=>word.push_str("three hundred"),
	400=>word.push_str("four hundred"),
	500=>word.push_str("five hundred"),
	600=>word.push_str("six hundred"),
	700=>word.push_str("seven hundred"),
	800=>word.push_str("eight hundred"),
	900=>word.push_str("nine hundred"),
	_=>()
	}
	if n/100> 0 && n%100>0 {
		word.push_str(" and ");
	}
	n=n%100;
	}
	if n >19 && n<100 {
		let m=(n/10)*10;
		
	match m {
	20=>word.push_str("twenty"),
	30=>word.push_str("thirty"),
	40=>word.push_str("forty"),
	50=>word.push_str("fifty"),
	60=>word.push_str("sixty"),
	70=>word.push_str("seventy"),
	80=>word.push_str("eighty"),
	90=>word.push_str("ninety"),
	_=>()
	}
	if n/10> 0 && n%10>0 {
		word.push('-');
	}
	n=n%10;
	}
	if n>=0 && n< 20
	{
	match n{
	0=>word.push_str("zero"),
	1=>word.push_str("one"),
	2=>word.push_str("two"),
	3=>word.push_str("three"),
	4=>word.push_str("four"),
	5=>word.push_str("five"),
	6=>word.push_str("six"),
	7=>word.push_str("seven"),
	8=>word.push_str("eight"),
	9=>word.push_str("nine"),
	10=>word.push_str("ten"),
	11=>word.push_str("eleven"),
	12=>word.push_str("twelve"),
	13=>word.push_str("thirteen"),
	14=>word.push_str("fourteen"),
	15=>word.push_str("fifteen"),
	16=>word.push_str("sixteen"),
	17=>word.push_str("seventeen"),
	18=>word.push_str("eighteen"),
	19=>word.push_str("nineteen"),
	_=>()
	}
	}
	return word;
}