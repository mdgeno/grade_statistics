use std::io;

fn main(){

	println!("Enter point totals, -1 stops:");
	
	let mut arr: [i32; 10] = [0; 10];
	let mut counter = 0;
	loop{
		
		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("enter valid input");

		let input: i32 = match input.trim().parse(){
				 Ok(num) => num,
				 Err(_) => continue
					    
				 };
		
		match input{
			0..=100 => {arr[counter] = input;
				    counter += 1},
			     -1 => break,
			      _ => continue
		};

		if counter==arr.len(){
			println!("Reached maximum valid input!");
			break;
		}
	}

	print_input(arr, counter);
	println!("Point average (all): {}", average(arr, counter.try_into().unwrap()));
	if passing(arr)==-1.0{
		println!("Point average (passing): -");
	}else{
		println!("Point average (passing): {}", passing(arr));
	};
	println!("Pass percentage: {}", pass_percentage(arr, counter.try_into().unwrap()));
}

fn print_input(arr: [i32; 10], current_index: usize){
	let mut i=0;
	while i<current_index{
		print!("{} ", arr[i]);
		i+=1;
	}
	println!(" ");
}

fn average(arr: [i32; 10], current_index: i32) -> f64{
	let mut total = 0;
	for grade in arr{
		total += grade;
	}
	
	(total/current_index).into()
}

fn passing(arr: [i32; 10]) -> f64{
	let mut passing_count = 0;
	let mut total = 0;
	for grade in arr{
		if grade>49&&grade<101{
			passing_count+=1;
			total+=grade;
		}
	}

	if total==0{
		return -1.0;
	}else{
		(total/passing_count).into()
	}
}

fn pass_percentage(arr: [i32; 10], current_count: i32) -> f64{
	let mut passing_count = 0;
	for grade in arr{
		if grade>49&&grade<101{
			passing_count+=1;
		}
	}

	if passing_count==0{
		return 0.0;
	}else{
		(100*passing_count/current_count).into()
	}
}

/* look pseudo
fn print_star(arr: [i32; 10], grade_category: i32){
	for grade in arr{
		if grade>=90{
			print!("*");
		}
	}
}
*/















