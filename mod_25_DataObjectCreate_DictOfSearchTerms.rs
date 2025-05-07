// IMPORT MODULES
use indexmap::IndexMap;

// FUNCTION() #25 - CREATE DATA OBJECT: DictOfSearchTerms
pub fn fn_DataObjectsCreate(
    ListOfSearchTerms: Vec<String>,
    ListOfSearchTermsWithSpaces: Vec<String>,
    _NumberOfSearchTerms: u32,
) -> (IndexMap<u32, String>, IndexMap<u32, String>) {
    
    /*
    MODULE.FUNCTION() #25 - CREATE DATA OBJECT: DictOfSearchTerms
    */
    
    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("WITHIN FUNCTION:  BEGIN FUNCTION #25 - CREATE DATA OBJECT: DictOfSearchTerms;");
    
    // DECLARE VARIABLES
    let mut DictOfSearchTermsWithSpaces: IndexMap<u32, String> = IndexMap::new(); // EMPTY DICT TO HOLD SEARCH TERMS
    let mut DictOfSearchTerms: IndexMap<u32, String> = IndexMap::new(); // EMPTY DICT TO HOLD SEARCH TERMS
    let mut k: u32 = 1; // INITIALIZE K KEY K/COUNTER
    
    // GET USER INPUT
    println!("\n");  // PRINT SPACE
    println!("Number of Search Terms: {}", _NumberOfSearchTerms);
    
    // BEGIN FOR LOOP - NO SPACES
    // FOR EACH IN NUMBER OF SEARCH TERMS INPUTTED BY USER
    for each in ListOfSearchTerms {
        
        // ADD ELS SEARCH TERM TO DICT OF SEARCH TERMS
        DictOfSearchTerms.insert(k, each);
        
        // INCREMENT THE K KEY K/COUNTER
        k += 1;
    }
    // END FOR LOOP
    
    // RESET K
    k = 1;
    
    // BEGIN FOR LOOP - WITH SPACES
    // FOR EACH IN NUMBER OF SEARCH TERMS INPUTTED BY USER
    for each in ListOfSearchTermsWithSpaces {
        
        // ADD ELS SEARCH TERM TO DICT OF SEARCH TERMS
        DictOfSearchTermsWithSpaces.insert(k, each);
        
        // INCREMENT THE K KEY K/COUNTER
        k += 1;
    }
    // END FOR LOOP
    
    // TEST PRINT OUTPUT - NO SPACES
    println!("\n");  // PRINT SPACE
    println!("You have inputted the following ELS Search Terms (NO SPACES - IF ANY):  ");
    println!("{:?}", DictOfSearchTerms);
    
    // TEST PRINT OUTPUT - WITH SPACES
    println!("\n");  // PRINT SPACE
    println!("You have inputted the following ELS Search Terms (WITH SPACES - IF ANY):  ");
    println!("{:?}", DictOfSearchTermsWithSpaces);
    
    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("WITHIN FUNCTION:  END FUNCTION #25 - CREATE DATA OBJECT: DictOfSearchTerms;");
    
    // RETURN VARIABLES TO PROGRAM
    return (DictOfSearchTerms, DictOfSearchTermsWithSpaces)
}

// END FUNCTION() #25 - CREATE DATA OBJECT: DictOfSearchTerms