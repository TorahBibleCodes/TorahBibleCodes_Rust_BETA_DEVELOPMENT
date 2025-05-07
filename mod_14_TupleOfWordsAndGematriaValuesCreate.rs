// IMPORT MODULES
use indexmap::IndexMap;

// FUNCTION() #14 - TUPLE OF WORDS AND GEMATRIA VALUES CREATE
pub fn fn_TupleOfWordsAndGematriaValuesCreate(
    ListOfWords: Vec<String>,
    NW: Vec<(u32, Vec<u32>, u32)>,
    ListOfIndexesCustom: Vec<u32>,
    ListOfIndexes4LettersInEachWord: Vec<Vec<u32>>
) -> (Vec<(String, Option<Vec<u32>>, (u32, Vec<u32>, u32))>, IndexMap<u32, (String, Option<Vec<u32>>, (u32, Vec<u32>, u32))>) {
    
    /*
    MODULE.FUNCTION() #14 - DATA OBJECT CREATE - RETURNS TUPLE OF WORDS WITH EACH WORD'S GEMATRIA NUMBER VALUE
    */
    
    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("WITHIN FUNCTION:  BEGIN FUNCTION #14 - TUPLE OF WORDS AND GEMATRIA VALUES CREATE");
          
    // DECLARE VARIABLES
    // CALL RUST BUILT-IN FUNCTION(S) - CREATE TUPLE OF WORDS, LETTER POSITIONS, WORD#, GEMATRIA VALUES FOR LETTER, GEMATRIA VALUE SUM FOR ENTIRE WORD
    let mut W: Vec<(String, Option<Vec<u32>>, (u32, Vec<u32>, u32))> = Vec::new();
    
    // CREATE W
    if ListOfIndexes4LettersInEachWord.is_empty() { // IF THE PARAMETERS ARE FOR THE ELSs, THEN WE USE THE DEFAULT VALUE OF EMPTY LIST
        
        // ZIP ListOfWords AND NW
        for (word, nw) in ListOfWords.into_iter().zip(NW) {
            W.push((word, None, nw));
        }
        
    } else { // ELSE WE ARE PASSING THE ENTIRE TEXT, AND THEREFORE NEED THE INDEX POSITIONS OF EACH LETTER
        
        // ZIP ListOfWords, ListOfIndexes4LettersInEachWord, AND NW
        for ((word, indices), nw) in ListOfWords.into_iter().zip(ListOfIndexes4LettersInEachWord).zip(NW) {
            W.push((word, Some(indices), nw));
        }
    }
    
    // CREATE DICT OF W
    let DW: IndexMap<u32, (String, Option<Vec<u32>>, (u32, Vec<u32>, u32))> = ListOfIndexesCustom
        .into_iter()
        .zip(W.iter().cloned())
        .collect();
    
    // TEST PRINT OUTPUT        
    // print(W)
    // println!("{:?}", W);
    
    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("WITHIN FUNCTION:  END FUNCTION #14 - TUPLE OF WORDS AND GEMATRIA VALUES CREATE");
    
    // RETURN VARIABLES - TUPLE OF WORDS AND GEMATRIA VALUES
    return (W, DW);
}

// END FUNCTION() #14 - TUPLE OF WORDS AND GEMATRIA VALUES CREATE