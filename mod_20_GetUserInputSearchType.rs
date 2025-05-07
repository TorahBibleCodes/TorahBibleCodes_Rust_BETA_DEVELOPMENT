// IMPORT MODULES
use std::io;
use std::io::Write;

// FUNCTION() #20 - GET USER INPUT: TYPE OF SEARCH INPUT
#[allow(unused_parens)]
pub fn fn_GetUserInput() -> u32 {
    
    /*
    MODULE.FUNCTION() #20 - GET USER INPUT: TYPE OF SEARCH INPUT ## RETURNS TypeOfSearchInput
    */
    
    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("WITHIN FUNCTION:  BEGIN FUNCTION #20 - GET USER INPUT: TYPE OF SEARCH INPUT ## RETURNS TypeOfSearchInput;");
    
    // GET USER INPUT - TYPE OF INPUT
    println!("\n");  // PRINT SPACE
    println!("Please select the type of input for ELS Search Terms:");
    println!("\n");  // PRINT SPACE
    println!("1 - Manual Input of ELS Search Term(s) via keyboard (i.e. type OR copy/paste Hebrew one-by-one)");
    println!("2 - Import Multiple ELS Search Term(s) from CSV file (i.e. list of one OR more Hebrew search terms in CSV file)");
    
    // TEXT CHOSEN = USER INPUT (TEXT STRING)
    println!("\n");  // PRINT SPACE
    print!("Please select type of input:  ");
    
    // Flush stdout to ensure prompt appears before input
    std::io::stdout().flush().expect("Failed to flush stdout");
    
    let mut TextString = String::new();
    io::stdin()
        .read_line(&mut TextString)
        .expect("Failed to read input");
    
    // Trim whitespace and convert to u32
    let NumberOfInputType: u32 = TextString
        .trim()
        .parse()
        .expect("Please enter a valid positive integer");
    
    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("You have chosen input type number # {}.", NumberOfInputType);
    
    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("WITHIN FUNCTION:  END FUNCTION #20 - GET USER INPUT: TYPE OF SEARCH INPUT ## RETURNS TypeOfSearchInput;");
    
    // RETURN VARIABLES TO PROGRAM
    return(NumberOfInputType);
}

// END FUNCTION() #20 - GET USER INPUT: TYPE OF SEARCH INPUT