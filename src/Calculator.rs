// This program is a basic calculator that can add, subtract, multiply and divide floating point numbers
// To use, after a number has been enter press enter, then enter an operator and press enter etc.
// The sum total is printed out after an '=' has been entered

use std::io;

fn main()
{
   let mut sum = 0f64;
   let mut flag = false;
   let mut operator: char = '_';

   loop
   {
      let input_term = term_input();

      // For the first loop sum holds the value of the first term
      // operation() is skipped to get the second term on the
      // second pass so all 3 parameters can be passed to sum_total()
      if(flag == false)
      {
         sum = input_term;
         flag = true;
         operator = operation();
         continue;
      }

      sum = sum_total(sum, input_term, operator);

      operator = operation();

      if(operator == '=')
      {
          println!("{}", sum);
          break;
      }
   }
}

// Read in Terms
fn term_input() -> f64
{
	let mut input = String::new();

	// .read_line() reads in a line of text up to \n.
	// .ok(), .expect() have to be called after
	// .read_line() to explicitly handle an error
    io::stdin().read_line(&mut input)
                .ok()
                .expect("Failed to read line");

    // Convert from string to float
    // .trim() removes white space
   	let input: f64 = match input.trim()
                                .parse()
   	{
    		Ok(num) => num,
    		Err(_) => panic!(),
	};
   	input
}

// Read in Operator
fn operation() -> char
{
	let mut operator_string = String::new();
   	io::stdin().read_line(&mut operator_string)
   			 .ok()
   			 .expect("Failed to read line");

    // .unwrap() handles errors and can
    // quickly destructure <Option>'s
   	let mut operator: char = operator_string.chars()
                                            .next()
                                            .unwrap();	
   												

   	operator
}

fn sum_total(mut sum: f64, input_term: f64, operator: char) -> f64
{
   match operator
   {
      '+' => sum = sum + input_term,
      '-' => sum = sum - input_term,
      '/' => sum = sum / input_term,
      '*' => sum = sum * input_term,
      '=' => {},
       _  => println!("Invalid Operator"),
   };

   sum
}

