// IMPORT MODULES

// FUNCTION() #17 - GET LIST OF FACTORS
pub fn fn_GetListOfFactors(LengthOfTextToSearch: u32) -> Vec<u32> {
    
    /*
    MODULE.FUNCTION() #17 - RETURNS LIST OF INTEGERS/FACTORS/DIVISORS OF THE LENGTH OF THE SELECTED TEXT
    */
    
    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("WITHIN FUNCTION:  BEGIN FUNCTION #17 - GET LIST OF FACTORS");
          
    // DECLARE VARIABLES
    let mut ListOfFactors: Vec<u32> = Vec::new(); // EMPTY LIST

    // BEGIN FOR LOOP
    for each in 1..=LengthOfTextToSearch {
            
        // IF THE LENGTH OF THE TEXT TO SEARCH IS EVENLY DIVISIBLE BY A NUMBER,
        if LengthOfTextToSearch % each == 0 {
            
            // THEN THAT NUMBER IS A FACTOR OF THE LENGTH OF THE TEXT TO SEARCH
            ListOfFactors.push(each); // APPEND POSITIVE FACTOR
        }
        // OTHERWISE DO NOTHING TO THAT NUMBER
        // else block is implicit in Rust
    }
    // END FOR LOOP

    // TEST PRINT OUTPUT
    println!("The are {} perfect factors in the text(s) that you selected: {:?}", ListOfFactors.len(), ListOfFactors);

    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("WITHIN FUNCTION:  END FUNCTION #17 - GET LIST OF FACTORS");
    
    // RETURN LIST OF FACTORS
    return ListOfFactors;
}

// END FUNCTION() #17 - GET LIST OF FACTORS