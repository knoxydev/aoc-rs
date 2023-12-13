use std::fs::File;
use std::io::{self, BufRead};


//The newly-improved calibration document consists of lines of text;
//each line originally contained a specific calibration value that the Elves now need to recover.
//On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.


fn read_line(x: String, nums: &[String; 9]) -> i64
{
	// GET NUMS FROM STRING
	let count: String = x.chars().fold(
		String::new(), |mut sum, val| {
			if nums.contains(&val.to_string()) { sum.push(val) }
			sum
	});

	// GET FIRST AND LAST DIGITS FROM NUMBERS
	let result_string: String =
	{
		let first_char = match count.chars().next() { Some(c) => c.to_string(), None => String::new(), };
		let last_char = match count.chars().last() { Some(c) => c.to_string(), None => String::new(), };
		format!("{}{}", first_char, last_char)
	};

	// CONVERT STRING TO NUMBER
	let res: i64 = match result_string.parse::<i64>() { Ok(num) => num, Err(num) => 0, };

	return res;
}


pub fn start() -> io::Result<()>
{
	let nums: [String; 9] = [
		"1".to_string(), "2".to_string(),
		"3".to_string(), "4".to_string(),
		"5".to_string(), "6".to_string(),
		"7".to_string(), "8".to_string(),
		"9".to_string()
	];


	let file = File::open("./data/day_1.txt")?;
	let reader = io::BufReader::new(file);

	let lines = reader.lines().filter_map(|line|
	{
		match line
		{
			Ok(line) => Some(line),
			Err(err) => {
				println!("ERROR: reading line");
				None
			}
		}
	});


	let count = lines.fold(0, |sum, val| sum + read_line(val, &nums));
	println!("-- Day 1 --\nRESULT: {count:?}");

	Ok(())
}
