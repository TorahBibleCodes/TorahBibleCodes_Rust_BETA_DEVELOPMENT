// IMPORT MODULES
use std::io;
use std::io::Write;
use indexmap::IndexMap;

// FUNCTION() #22 - GET USER INPUT: INPUT DESIRED SEARCH TERMS
pub fn fn_GetUserInput(NumberOfSearchTerms: u32) -> (Vec<String>, Vec<String>, IndexMap<u32, String>, IndexMap<u32, String>) {
    
    /*
    MODULE.FUNCTION() #22 - GET USER INPUT: INPUT DESIRED SEARCH TERMS ListOfSearchTerms, DictOfSearchTerms
    */
    
    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("WITHIN FUNCTION:  BEGIN FUNCTION #22 - GET USER INPUT - ELS SEARCH TERMS;");
    
    // DECLARE VARIABLES
    let mut ListOfSearchTermsWithSpaces: Vec<String> = Vec::new(); // EMPTY LIST
    let mut DictOfSearchTermsWithSpaces: IndexMap<u32, String> = IndexMap::new(); // EMPTY DICT TO HOLD SEARCH TERMS
    let mut ListOfSearchTerms: Vec<String> = Vec::new(); // EMPTY LIST
    let mut DictOfSearchTerms: IndexMap<u32, String> = IndexMap::new(); // EMPTY DICT TO HOLD SEARCH TERMS
    let mut k: u32 = 1; // INITIALIZE K KEY K/COUNTER
    
    // GET USER INPUT
    println!("\n");  // PRINT SPACE
    println!("Number of ELS Search Terms: {}", NumberOfSearchTerms);
    println!("Please input each ELS Search Term (in Hebrew) for the {} terms that you specified\n", NumberOfSearchTerms);
    
    // BEGIN FOR LOOP
    // FOR EACH IN NUMBER OF SEARCH TERMS INPUTTED BY USER
    for each in 1..=NumberOfSearchTerms {
        
        // TEXT CHOSEN = USER INPUT (TEXT STRING)
        println!("\n");  // PRINT SPACE
        print!("ELS Search Term {} (type OR copy/paste Hebrew): ", each);
        
        // Flush stdout to ensure prompt appears before input
        std::io::stdout().flush().expect("Failed to flush stdout");
        
        let mut TextString = String::new();
        io::stdin()
            .read_line(&mut TextString)
            .expect("Failed to read input");
        
        // Trim whitespace for processing
        let TextString = TextString.trim();
        
        // TEST PRINT OUTPUT
        println!("\n");  // PRINT SPACE
        println!("ELS Search Term (type OR copy/paste Hebrew): {}", TextString);
        // Reverse the string for right-to-left display
        let reversed: String = TextString.chars().rev().collect();
        println!("ELS Search Term (R-T-L): {}", reversed);
        
        // ASSIGN VALUE OF TEXT STRING TO VARIABLE
        let ELSSearchTermTextWithSpaces = TextString.to_string();
        
        // APPEND ELS SEARCH TERM TEXT TO LIST OF SEARCH TERMS WITH SPACES
        ListOfSearchTermsWithSpaces.push(ELSSearchTermTextWithSpaces.clone());
        
        // ADD ELS SEARCH TERM TO DICT OF SEARCH TERMS WITH SPACES
        DictOfSearchTermsWithSpaces.insert(k, ELSSearchTermTextWithSpaces.clone());
        
        // DEAL WITH SPACES IN ELS SEARCH TERM - REMOVE SPACES FROM STRING
        let ELSSearchTermText = ELSSearchTermTextWithSpaces.replace(" ", "");
        
        // APPEND ELS SEARCH TERM TEXT TO LIST OF SEARCH TERMS
        ListOfSearchTerms.push(ELSSearchTermText.clone());
        
        // ADD ELS SEARCH TERM TO DICT OF SEARCH TERMS
        DictOfSearchTerms.insert(k, ELSSearchTermText);
        
        // INCREMENT THE K KEY K/COUNTER
        k += 1;
    }
    // END FOR LOOP
    
    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("You have entered the following ELS search terms (NO SPACES - IF ANY):  {:?}", ListOfSearchTerms);
    println!("{:?}", DictOfSearchTerms);
    
    println!("\n");  // PRINT SPACE
    println!("You have entered the following ELS search terms (WITH SPACES - IF ANY):  {:?}", ListOfSearchTermsWithSpaces);
    println!("{:?}", DictOfSearchTermsWithSpaces);
    
    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("WITHIN FUNCTION:  END FUNCTION #22 - GET USER INPUT - ELS SEARCH TERMS;");
    
    // RETURN VARIABLES TO PROGRAM
    return(ListOfSearchTerms, ListOfSearchTermsWithSpaces, DictOfSearchTerms, DictOfSearchTermsWithSpaces);
}

// END FUNCTION() #22 - GET USER INPUT: INPUT DESIRED SEARCH TERMS