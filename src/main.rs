use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Function with combinator
/// 
/// The main goal is to return a Result type, because of we want to know the result 
/// or the error description. That is the main difference between Option and Result.
/// Some map here is using for converting Option type to Result type
///
fn file_double_w_combinator<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
	File::open(file_path)
		.map_err(|err| err.to_string())
		.and_then(|mut file| {
			let mut contents = String::new();
			file.read_to_string(&mut contents)
				.map_err(|err| err.to_string())
				.map(|_| contents)
		})
		.and_then(|contents| {
			contents.trim().parse::<i32>()
				.map_err(|err| err.to_string())
		})
		.map(|n| 2*n)
}

/// Function with combinator and macro
///
/// A little simplier but the same as above
///
fn file_double_w_macro<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
	let mut file = File::open(file_path).map_err(|e| e.to_string())?;
	let mut contents = String::new();
	file.read_to_string(&mut contents).map_err(|e| e.to_string())?;
	let n = contents.trim().parse::<i32>().map_err(|e| e.to_string())?;
	Ok(2*n)
}

/// Function with return
///
/// The same as above, but without combinators
///
fn file_double_w_return<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
	let mut file = match File::open(file_path) {
		Ok(file) => file,
		Err(err) => return Err(err.to_string()),
	};
	
	let mut contents = String::new();
	if let Err(err) = file.read_to_string(&mut contents) {
		return Err(err.to_string());
	}
	
	let n: i32 = match contents.trim().parse() {
		Ok(n) => n,
		Err(err) => return Err(err.to_string()),
	};
	
	Ok(2*n)
}

fn main() {
    match file_double_w_combinator("test.txt") {
		Ok(n) => println!("{}", n),
		Err(err) => println!("Error: {}", err),
	}
	
    match file_double_w_return("test.txt") {
		Ok(n) => println!("{}", n),
		Err(err) => println!("Error: {}", err),
	}	
	
    match file_double_w_macro("test.txt") {
		Ok(n) => println!("{}", n),
		Err(err) => println!("Error: {}", err),
	}	
}
