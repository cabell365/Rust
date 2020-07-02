// Libraries in rust
use std::io;
// use std::{i32};
use std::collections::HashMap;
use rand::{thread_rng, Rng};

fn main() {
	
	// min and max value of a die
	let my_min_die_val = 1;
	let my_max_die_val = 6;
	
	//My score dictionary
	let mut my_points_dict =  HashMap::new();
	my_points_dict. insert (0,0);
	my_points_dict. insert (1,5);
	my_points_dict. insert (2,10);
	my_points_dict. insert (3,15);
	my_points_dict. insert (4,20);
	my_points_dict. insert (5,25);
	
	// My message dictionary
	let mut my_points_message_dict =  HashMap::new();
	my_points_message_dict. insert (0,"You have no luck at all.");
	my_points_message_dict. insert (1,"You have a tiny bit of luck.");
	my_points_message_dict. insert (2,"You luck level is rising. ");
	my_points_message_dict. insert (3,"You could be on a roll!");
	my_points_message_dict. insert (4,"You are a very lucky person!");
	my_points_message_dict. insert (5,"You should play the lottery!");


	// print information to the player
	println!("You score points for every time you roll your lucky number.");
	println!("A die is removed each time your lucky number is rolled.");
	println!("Here is the scoring for the game");
	println!("* 5 of your lucky number: 25 points");
	println!("* 4 of your lucky number: 20 points");
	println!("* 3 of your lucky number: 15 points");
	println!("* 2 of your lucky number: 10 points");
	println!("* 1 of your lucky number: 5 points");
	
	//let my_roll_list = HashMap::new();
	
	// let mut my_winning_num = 0;
	let mut is_not_valid_num = true;
	while is_not_valid_num {
		println!("found");
		
		// Request for entering number 1
		println!("Please enter your lucky number between {} and {}:",my_min_die_val , my_max_die_val );
		let mut my_winning_num_str = String::new();
		io::stdin().read_line(&mut my_winning_num_str).expect("Failed to read line");
		
		// Converting string to integer
		let my_winning_num: i32 = my_winning_num_str.trim().parse().ok().expect("Program only processes numbers, Enter number");
		println!("The value of num is {}",my_winning_num);

		if my_winning_num >  my_min_die_val && my_winning_num < my_max_die_val {
			is_not_valid_num = false;
		}
	}
	// println!("The value is {}",my_winning_num);
	
	
	
	// Iterate over dice list.
	for (die, score) in &my_points_dict {
		println!("{}: \"{}\"", die, score);
	}
	
	// initialize a vector for dice values
    let my_dice_values = [1,2,3,4,5,6];
    println!("Initial vector: {:?}", my_dice_values);
    
	// The 'len' method yields the number of elements currently stored in a vector
	println!("Vector length: {}", my_dice_values.len());
	
	// Vector's can be easily iterated over
    println!("Contents of my_dice_values:");
    for x in my_dice_values.iter() {
        println!("> {}", x);
    }
	
    // Create random number beteen two values.
	let mut rng = thread_rng();
	println!("Die Value: {}", rng.gen_range(my_min_die_val , my_max_die_val ));
}
