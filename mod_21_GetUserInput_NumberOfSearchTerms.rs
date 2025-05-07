// IMPORT MODULES
use std::io;
use std::io::Write;

// FUNCTION() #21 - GET USER INPUT: NUMBER OF SEARCH TERMS
pub fn fn_GetUserInput() -> u32 {
    
    /*
    MODULE.FUNCTION() #21 - GET USER INPUT: NUMBER OF SEARCH TERMS ## RETURNS NumberOfSearchTerms
    */
    
    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("WITHIN FUNCTION:  BEGIN FUNCTION #21 - GET USER INPUT - NUMBER OF SEARCH TERMS;");
    
    // GET USER INPUT ## TEXT CHOSEN = USER INPUT (TEXT STRING)
    println!("\n");  // PRINT SPACE
    print!("How many total number of ELS Search-Terms would you like to search for within the selected text?:\n");
    
    // Flush stdout to ensure prompt appears before input
    std::io::stdout().flush().expect("Failed to flush stdout");
    
    let mut TextString = String::new();
    io::stdin()
        .read_line(&mut TextString)
        .expect("Failed to read input");
    
    // CONVERT TEXT STRING TO INTEGER
    let NumberOfSearchTerms: u32 = TextString
        .trim()
        .parse()
        .expect("Please enter a valid positive integer");
    
    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("You have chosen to search for {} ELS Search Terms", NumberOfSearchTerms);
    
    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("WITHIN FUNCTION:  END FUNCTION #21 - GET USER INPUT - NUMBER OF SEARCH TERMS;");
    
    // RETURN VARIABLES TO PROGRAM
    return NumberOfSearchTerms;
}

// END FUNCTION() #21 - GET USER INPUT: NUMBER OF SEARCH TERMS