use std::io;
use rand::Rng;  // scope rand funct(s)
use std::cmp::Ordering; // an enum {Less, Greater, Equal}

fn main() {
    println!("Guess the number!");


    let secret_number = rand::thread_rng().gen_range(1, 101); // (inclusive, exclusive)

    loop { // infinite loop
        println!("Please enter your guess");
                                // essentially, a static method
        let mut guess = String::new(); // calls associated new function in String
                                       // associated type function call, not an instance

        // grab stdin handle, call method
        // add mut qualifier since references are immutable by default
        io::stdin().read_line(&mut guess) // returns Result enum
            .expect("Failed to read line");
        // expect will check for Ok or Err, Err being a failed op
        // io::Result.expect(...) will crash/abort if Result is Err

        // === convert ====
        let guess: u32 = match guess.trim().parse() { // set arms on return for Result
            Ok(num) => num, // match Ok, return success value
            Err(_) => continue, // restart loop if Err, '_' catches all
        }; // <-- notice the required ';'
        // shadows guess from above
        // trim eliminates whitespace and newlines
        // ':' annotates var type


        println!("You guessed: {}", guess); // {} acts as a placeholder for guess
        // === similarly ===
        // let x = 5;
        // let y = 10;
        // println!("x = {}, y = {}", x, y);

        match guess.cmp(&secret_number) { // match finds which arm pattern matches
            Ordering::Less => println!("Too small!"), // executes code if matched
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // stop the infinite loop
            }
        }
    }
}
