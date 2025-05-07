// IMPORT MODULES

// FUNCTION() #16 - GET LENGTH OF TEXT TO SEARCH
#[allow(unused_parens)]
pub fn fn_GetLengthOfTextToSearch(L: &Vec<char>) -> u32 {
    
    /*
    MODULE.FUNCTION() #16 - RETURNS INTEGER OF THE LENGTH OF THE SELECTED TEXT
    */
    
    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("WITHIN FUNCTION:  BEGIN FUNCTION #16 - GET LENGTH OF TEXT TO SEARCH");
          
    // DECLARE VARIABLES
    let LengthOfTextToSearch = L.len() as u32;
    // println!("Length of Text to Search = {}", LengthOfTextToSearch);
    
    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("WITHIN FUNCTION:  END FUNCTION #16 - GET LENGTH OF TEXT TO SEARCH");
    
    // RETURN VARIABLES - LENGTH OF TEXT TO SEARCH
    return(LengthOfTextToSearch); // RETURNS INTEGER
    
}

// END FUNCTION() #16 - GET LENGTH OF TEXT TO SEARCH