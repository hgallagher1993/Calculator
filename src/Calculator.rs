use std::old_io;

fn main()
{
   let mut sum = 0f64;
   let mut flag = false;
   let mut operator: char = '_';

   loop
   {
      let input_term = term_input();

      // Destructure input<f64>
      let term = match input_term
      {
         Some(term) => term,
         None		=>
         {
           println!("Enter a number");
           return;
         }
      };

      // For the first loop sum holds the value of the first term
      // operation() is skipped to get the second term on the
      // second pass so all 3 parameters can be passed to sum_total()
      if(flag == false)
      {
         sum = term;
         flag = true;
         operator = operation();
         continue;
      }

      sum = sum_total(sum, &term, &operator);

      operator = operation();

      if(operator == '=')
      {
          println!("{}", sum);
          break;
      }
   }
}

// Read in Terms
fn term_input() -> Option<f64>
{
   let input = old_io::stdin()
              .read_line() // Return a full line of input.
              .ok() // Handles .read_line() return type (Like match below)
              .expect("Failed to read line"); // Prints message on invalid input

   // Return Input
   let input_num: Option<f64> = input.trim() // Removes \n from string
                                     .parse() // Takes in a &str value and converts it to an Option<f64>
                                     .ok();
   input_num
}

// Read in Operator
fn operation() -> char
{
   let operator = old_io::stdin()
                     .read_line()
                     .ok()
                     .expect("Failed to read line")
                     .as_slice() // Takes a reference of input
                     .char_at(0); // converts position 0 to a Char

   // Return Operator
   operator
}

fn sum_total(mut sum: f64, term: &f64, operator: &char) -> f64
{
   match *operator
   {
      '+' => sum = sum + *term,
      '-' => sum = sum - *term,
      '/' => sum = sum / *term,
      '*' => sum = sum * *term,
      '=' => {},
       _  => println!("Invalid Operator"),
   };

   // Return Sum
   sum
}
