use std::fs::File;
use std::io::{self, BufRead};


#[derive(Debug)]
struct Element
{
	id : i64,
	games : Vec<Games>
}
#[derive(Debug)]
struct Games
{
	red : i64,
	blue : i64,
	green : i64,
}


fn get_colors(x: String) -> Games
{
	let parts: Vec<&str> = x.split(',').map(|s| s.trim()).collect();
	let mut games = Games { red : 0, blue : 0, green : 0, };

	for i in parts
	{
		if i.contains((&"red".to_string()))
			{ games.red = match i.replace(" red", "").parse::<i64>() { Ok(num) => num, Err(num) => 0, }; }
		else if i.contains((&"blue".to_string()))
			{ games.blue = match i.replace(" blue", "").parse::<i64>() { Ok(num) => num, Err(num) => 0, }; }
		else if i.contains((&"green".to_string()))
			{ games.green = match i.replace(" green", "").parse::<i64>() { Ok(num) => num, Err(num) => 0, }; }
	}

	return games;
}


fn get_games(line: String) -> Vec<Games>
{
	let mut games: String = String::new();
	let mut list: Vec<String> = Vec::new();
	let mut colors: Vec<Games> = Vec::new();

	{
		let lines: Vec<&str> = line.lines().collect();
		for x in &lines
			{ if let Some(colon_index) = x.find(':') { games = (&x[colon_index + 1..].trim()).to_string(); } }
	}

	let lines: Vec<&str> = games.lines().collect();
	for line in lines {
		let segments: Vec<&str> = line.split(';').collect();
		for x in segments { list.push(x.trim().to_string()); }
	}

	//println!("{:?}", list);

	for i in list { colors.push(get_colors(i)); }
	return colors;
}


pub fn start() -> io::Result<()>
{
	let mut base: Vec<Element> = Vec::new();
	let mut count: i64 = 0;
	let mut id: i64 = 0;


	let file = File::open("./data/day_2.txt")?;
	let reader = io::BufReader::new(file);
	let lines = reader.lines().filter_map(|line| {
		match line {
			Ok(line) => Some(line),
			Err(err) => { println!("ERROR: reading line"); None }
		}
	});


	for i in lines
	{
		id += 1;
		base.push(Element { id : id, games : get_games(i), });
	}


	for elem in &base
	{
		let conditions_met = elem.games.iter().all(|game|
			{ game.red > 12 || game.blue > 14 || game.green > 13 });

		if conditions_met == true { continue; }
		else { count += elem.id; }
	}


	println!("-- Day 2 --\nRESULT: {:?}", count);
	Ok(())
}