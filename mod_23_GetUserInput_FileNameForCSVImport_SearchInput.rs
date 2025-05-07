// IMPORT MODULES
use std::io;
use std::io::Write;

// FUNCTION() #23 - GET USER INPUT: CSV FILE NAME FOR SEARCH INPUT
pub fn fn_GetUserInput() -> String {
    
    /*
    MODULE.FUNCTION() #23 - GET USER INPUT: CSV FILE NAME FOR SEARCH INPUT ## RETURNS
    */
    
    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("WITHIN FUNCTION:  BEGIN FUNCTION #23 - GET USER INPUT: CSV FILE NAME FOR SEARCH INPUT ## RETURNS ;");
    
    // TEXT CHOSEN = USER INPUT (TEXT STRING)
    println!("\n");  // PRINT SPACE
    print!("Please input the CSV File Name (in the root directory where you are running this Torah Bible Codes search program) that contains your ELS Search Terms (e.g. FileName4ELSs.csv):  ");
    
    // Flush stdout to ensure prompt appears before input
    std::io::stdout().flush().expect("Failed to flush stdout");
    
    let mut TextString = String::new();
    io::stdin()
        .read_line(&mut TextString)
        .expect("Failed to read input");
    
    // ASSIGN TEXT STRING VALUE TO FILE NAME
    let FileNameForCSVImport = TextString.trim().to_string();
    
    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("The CSV File to be opened is {}.", FileNameForCSVImport);
    
    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("WITHIN FUNCTION:  END FUNCTION #23 - GET USER INPUT: CSV FILE NAME FOR SEARCH INPUT ## RETURNS ;");
    
    // RETURN VARIABLES TO PROGRAM
    return FileNameForCSVImport;
}

// END FUNCTION() #23 - GET USER INPUT: CSV FILE NAME FOR SEARCH INPUT