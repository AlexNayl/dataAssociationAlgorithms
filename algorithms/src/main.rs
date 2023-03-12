use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::env;
use std::time::Instant;
use std::fs::create_dir;

use crate::accuracy_test::accuracy_test;
pub mod fast_apriori;
pub mod pcy;
pub mod random_sample;
pub mod accuracy_test;
pub mod son;
pub mod mutli_pcy;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() != 2{
		println!("USAGE: Expects command line parameters: <DATA_PATH>");
		return;
	}

	//Load in the input file as text
	let mut data_raw = String::new();
	let mut input_file = File::open(&args[1]).expect("Data file cant be opened!");
	input_file.read_to_string(&mut data_raw).expect("The string cannot be read");


	_ = create_dir("./output"); //verify output directory exists

	//DATA INTAKE

	let mut data:Vec<Vec<u32>> = Vec::new();	//data is stored as a vector of vectors of unsigned 32bit integers
	let mut max_number:u32 = 0; 				//The max number will be used in several algorithms

	println!("Reading in data.");
	for line_str in data_raw.lines(){
		let mut line_vec:Vec<u32> = Vec::new();
		for cell_str in line_str.split_whitespace(){
			let cell = cell_str.parse::<u32>().expect("Non numerical encountered in data.");
			if cell > max_number{
				max_number = cell.clone();
			}
			line_vec.push(cell);
		}
		data.push(line_vec);
	}


	//Test Variables
	let proportions = [20,40,60,80,100];
	let confidences = [0.01,0.02,0.05];
	let support_threshold = 3;

	//Stores accurate results for comparisons lator
	let mut good_results = (HashMap::new(), HashMap::new());
	let mut file_header = "DataPercent ConfidenceThreshold Runtime";


	println!("------------Apirori---------------");
	let current_error_message = "Cannot write to output/apriori.dat";
	let mut f = File::create("output/apriori.dat").expect(current_error_message);
	writeln!(f,"{:}",file_header).expect(current_error_message);

	for prop in &proportions[1..proportions.len()]{
		write!(f,"{:} {:} ",prop, 0.05).expect(current_error_message);
		println!("Running {:} of data at support threshold {:} and confidence threshold {:}",*prop,support_threshold,0.05);
		let now = Instant::now();
		_ = fast_apriori::apriori(&data[0..((data.len()*prop) / 100)], support_threshold, 0.05);
		let time = now.elapsed();
		println!("Ran for {:}", time.as_secs_f32());
		writeln!(f,"{:} ",time.as_secs_f32()).expect(current_error_message);
	}
	for conf in &confidences{
		let prop = proportions[0];
		write!(f,"{:} {:} ",prop,conf).expect(current_error_message);
		println!("Running {:} of data at support threshold {:} and confidence threshold {:}",prop,support_threshold,*conf);
		let now = Instant::now();
		_ = fast_apriori::apriori(&data[0..((data.len()*prop) / 100)], support_threshold, *conf);
		let time = now.elapsed();
		println!("Ran for {:}", time.as_secs_f32());
		writeln!(f,"{:} ",time.as_secs_f32()).expect(current_error_message);
	}
	f.flush().expect(current_error_message);


	println!("------------PCY---------------");
	let current_error_message = "Cannot write to output/pcy.dat";
	let mut f = File::create("output/pcy.dat").expect(current_error_message);
	writeln!(f,"{:}",file_header).expect(current_error_message);

	for prop in &proportions[1..proportions.len()]{
		write!(f,"{:} {:} ",prop, 0.05).expect(current_error_message);
		println!("Running {:} of data at support threshold {:} and confidence threshold {:}",*prop,support_threshold,0.05);
		let now = Instant::now();
		_ = pcy::pcy(&data[0..((data.len()*prop) / 100)], max_number as usize, support_threshold, 0.05);
		let time = now.elapsed();
		println!("Ran for {:}", time.as_secs_f32());
		writeln!(f,"{:} ",time.as_secs_f32()).expect(current_error_message);
	}
	for conf in &confidences{
		let prop = proportions[0];
		write!(f,"{:} {:} ",prop,conf).expect(current_error_message);
		println!("Running {:} of data at support threshold {:} and confidence threshold {:}",prop,support_threshold,*conf);
		let now = Instant::now();
		_ = pcy::pcy(&data[0..((data.len()*prop) / 100)], max_number as usize, support_threshold, *conf);
		let time = now.elapsed();
		println!("Ran for {:}", time.as_secs_f32());
		writeln!(f,"{:} ",time.as_secs_f32()).expect(current_error_message);
	}
	f.flush().expect(current_error_message);

	println!("------------Multi PCY---------------");
	let current_error_message = "Cannot write to output/multipcy.dat";
	let mut f = File::create("output/multipcy.dat").expect(current_error_message);
	writeln!(f,"{:}",file_header).expect(current_error_message);

	for prop in &proportions[1..proportions.len()]{
		write!(f,"{:} {:} ",prop, 0.05).expect(current_error_message);
		println!("Running {:} of data at support threshold {:} and confidence threshold {:}",*prop,support_threshold,0.05);
		let now = Instant::now();
		good_results = mutli_pcy::pcy(&data[0..((data.len()*prop) / 100)], max_number as usize, support_threshold, 0.05);
		let time = now.elapsed();
		println!("Ran for {:}", time.as_secs_f32());
		writeln!(f,"{:} ",time.as_secs_f32()).expect(current_error_message);
	}
	for conf in &confidences{
		let prop = proportions[0];
		write!(f,"{:} {:} ",prop,conf).expect(current_error_message);
		println!("Running {:} of data at support threshold {:} and confidence threshold {:}",prop,support_threshold,*conf);
		let now = Instant::now();
		_ = mutli_pcy::pcy(&data[0..((data.len()*prop) / 100)], max_number as usize, support_threshold, *conf);
		let time = now.elapsed();
		println!("Ran for {:}", time.as_secs_f32());
		writeln!(f,"{:} ",time.as_secs_f32()).expect(current_error_message);
	}
	f.flush().expect(current_error_message);

	good_results.0.shrink_to_fit();
	good_results.1.shrink_to_fit();

	file_header = "DataPercent ConfidenceThreshold Runtime FalsePositives FalseNegatives";

	println!("------------Random Sample--------------");
	let current_error_message = "Cannot write to output/rand.dat";
	let mut f = File::create("output/rand.dat").expect(current_error_message);
	writeln!(f,"{:}",file_header).expect(current_error_message);
	let mut results:(HashMap<u32,u32>,HashMap<(u32,u32),u32>);

	for prop in &proportions[1..proportions.len()]{
		write!(f,"{:} {:} ",prop, 0.05).expect(current_error_message);
		println!("Running {:} of data at support threshold {:} and confidence threshold {:}",prop,support_threshold,0.05);
		let now = Instant::now();
		results=random_sample::random_sample(&data[0..((data.len()*prop) / 100)], 0.5, support_threshold, 0.05);
		let time = now.elapsed();
		println!("Ran for {:}", time.as_secs_f32());
		let (false_positives, false_negatives) = accuracy_test(&results, &good_results);
		writeln!(f,"{:} {:} {:}",time.as_secs_f32(), false_positives, false_negatives).expect(current_error_message);
	}
	for conf in &confidences{
		let prop = proportions[0];
		write!(f,"{:} {:} ",prop,conf).expect(current_error_message);
		println!("Running {:} of data at support threshold {:} and confidence threshold {:}",prop,support_threshold,*conf);
		let now = Instant::now();
		results = random_sample::random_sample(&data[0..((data.len()*prop) / 100)], 0.5, support_threshold, *conf);
		let time = now.elapsed();
		println!("Ran for {:}", time.as_secs_f32());
		let (false_positives, false_negatives) = accuracy_test(&results, &good_results);
		writeln!(f,"{:} {:} {:}",time.as_secs_f32(), false_positives, false_negatives).expect(current_error_message);
	}
	f.flush().expect(current_error_message);

	println!("------------SON--------------");
	let current_error_message = "Cannot write to output/son.dat";
	let mut f = File::create("output/son.dat").expect(current_error_message);
	writeln!(f,"{:}",file_header).expect(current_error_message);

	for prop in &proportions[1..proportions.len()]{
		write!(f,"{:} {:} ",prop, 0.05).expect(current_error_message);
		println!("Running {:} of data at support threshold {:} and confidence threshold {:}",*prop,support_threshold,0.05);
		let now = Instant::now();
		results=son::son(&data[0..((data.len()*prop) / 100)], 100, support_threshold, 0.05);
		let time = now.elapsed();
		println!("Ran for {:}", time.as_secs_f32());
		let (false_positives, false_negatives) = accuracy_test(&results, &good_results);
		writeln!(f,"{:} {:} {:}",time.as_secs_f32(), false_positives, false_negatives).expect(current_error_message);
	}
	for conf in &confidences{
		let prop = proportions[0];
		write!(f,"{:} {:} ",prop,conf).expect(current_error_message);
		println!("Running {:} of data at support threshold {:} and confidence threshold {:}",prop,support_threshold,*conf);
		let now = Instant::now();
		results = son::son(&data[0..((data.len()*prop) / 100)], 100, support_threshold, *conf);
		let time = now.elapsed();
		println!("Ran for {:}", time.as_secs_f32());
		let (false_positives, false_negatives) = accuracy_test(&results, &good_results);
		writeln!(f,"{:} {:} {:}",time.as_secs_f32(), false_positives, false_negatives).expect(current_error_message);
	}
	f.flush().expect(current_error_message);


}
