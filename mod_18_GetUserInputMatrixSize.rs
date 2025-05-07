// IMPORT MODULES
use std::io;
use std::io::Write;

// FUNCTION() #18 - GET USER INPUT: SIZE OF 2D MATRIX
pub fn fn_GetUserInput(ListOfFactors: &Vec<u32>, LengthOfTextToSearch: u32) -> (u32, u32) {
    
    /*
    MODULE.FUNCTION() #18 - GET USER INPUT: SIZE OF 2D MATRIX; RETURNS y, x
    */
    
    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("WITHIN FUNCTION:  BEGIN FUNCTION #18 - GET USER INPUT: SIZE OF 2D MATRIX;");
    
    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("LengthOfTextToSearch = {}", LengthOfTextToSearch);
    println!("\n");  // PRINT SPACE
    println!("List of {} Factors from the text(s) that you selected: {:?}", ListOfFactors.len(), ListOfFactors);
    println!("Choose a size for the 2D matrix from a number from the List of {} Factors (i.e. # of x columns to output for the 2D Matrix)", ListOfFactors.len());
    println!("\n");
    println!("OR Choose ANY number, and the 2D Matrix will be calculated according to your selection.");
    println!("\n");
    println!("NOTE: Larger numbers above 800 for X / Width / Columns may (or may not) exceed the maximum allowed by Microsoft Office (Excel), LibreOffice, Apache Open Office, etc.");
    
    // TEXT CHOSEN = USER INPUT (TEXT STRING)
    println!("\n");  // PRINT SPACE
    print!("Type the number here: ");
    
    // Flush stdout to ensure prompt appears before input
    std::io::stdout().flush().expect("Failed to flush stdout");
    
    let mut TextString = String::new();
    io::stdin()
        .read_line(&mut TextString)
        .expect("Failed to read input");
    
    // Trim whitespace and convert to u32
    let x: u32 = TextString
        .trim()
        .parse()
        .expect("Please enter a valid positive integer");
    
    // TEST PRINT OUTPUT
    println!("Size of 2D Matrix : {} columns", x);
    
    // CALCULATE CUSTOM Y VALUE; TRUNCATED BY MAKING INTEGER
    let y: u32 = LengthOfTextToSearch / x;

    // TEST PRINT OUTPUT
    println!("Size of 2D Matrix : {} rows", y);
    
    // TEST
    // y += 1; // Commented out as in Python
    
    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("WITHIN FUNCTION:  END FUNCTION #18 - GET USER INPUT: SIZE OF 2D MATRIX;");
    
    // RETURN VARIABLES TO PROGRAM
    return(y, x);
}

// END FUNCTION() #18 - GET USER INPUT: SIZE OF 2D MATRIX