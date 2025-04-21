// IMPORT MODULES

/// A module for creating a data structure that maps letter positions to words in a Hebrew text sequence.
///
/// MODULE.FUNCTION() #7 - DATA OBJECTS CREATE; RETURNS -> LIST OF LISTS OF LETTER POSITION INDEXES
///
/// This module provides functionality to process a list of Hebrew words and generate a vector of vectors, where
/// each inner vector contains the position indexes of the letters in a corresponding word. It creates a data structure
/// that tracks the sequential position of each letter within the entire text sequence for each word.
///
/// This function iterates through each word in the provided list, processes each letter in the word, and assigns a
/// sequential position index to each letter based on its order in the text. Each word's letter positions are collected
/// into a temporary vector, which is then added to the result vector. The function ensures that letter positions are
/// tracked continuously across all words.
///
/// # Arguments
/// * `ListOfWords` - A `Vec<String>` containing the sequence of Hebrew words to process.
///
/// # Returns
/// A `Vec<Vec<u32>>` where:
/// - Each inner `Vec<u32>` represents a word and contains the sequential position indexes (starting from 1) of each letter in that word.
/// - The outer `Vec` contains one inner vector of integers per word from the input `ListOfWords`.
///
/// Example input `ListOfWords`:
/// - `["בראשית", "ברא", "אלהים"]`
///
/// Example output:
/// - `[[1, 2, 3, 4, 5, 6, 7], [8, 9, 10], [11, 12, 13, 14, 15]]`
///   - First word "בראשית" has letters at positions 1 to 7.
///   - Second word "ברא" has letters at positions 8 to 10.
///   - Third word "אלהים" has letters at positions 11 to 15.
///
/// # Examples
/// ```
/// // CALL MODULE.FUNCTION() #7 - DATA OBJECTS CREATE; RETURNS -> LIST OF LISTS OF LETTER POSITION INDEXES
/// let ListOfIndexes4LettersInEachWord = mod_7_DataObjectsCreate::fn_DataObjectsCreate(ListOfWords);
/// ```
///
/// # Panics
/// This function does not panic under normal circumstances. It assumes valid input data (a list of strings containing
/// valid Unicode Hebrew characters) and processes each word and letter sequentially.
///
/// MODULE.FUNCTION() #7 - DATA OBJECTS CREATE; RETURNS -> LIST OF LISTS OF LETTER POSITION INDEXES
/// 
// BEGIN FUNCTION() #7 - DATA OBJECTS CREATE
pub fn fn_DataObjectsCreate(ListOfWords: Vec<String>) -> Vec<Vec<u32>> {
    
    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("WITHIN FUNCTION:  BEGIN FUNCTION #7 - DATA OBJECTS CREATE");
    
    // DECLARE VARIABLES 
    let mut LetterCounter = 1;
    let mut ListOfIndexes4LettersInEachWord: Vec<Vec<u32>> = Vec::new();
    let mut ListTemporary: Vec<u32> = Vec::new();

    // BEGIN FOR LOOP
    // FOR EACH WORD IN ListOfWords...
    for EachWord in ListOfWords { // EACH WORD

        // BEGIN FOR LOOP
        for EachLetter in EachWord.chars() { // EACH LETTER

            //
            ListTemporary.push(LetterCounter);

            // INCREMENT LETTER COUNTER
            LetterCounter += 1;
        }
        // END FOR LOOP

        // ADD EACH CURRENT TOTAL-LETTER-COUNT-VALUE-INDEX-POSITION (I.E. EACH LETTER)
        ListOfIndexes4LettersInEachWord.push(ListTemporary);

        // RESET TEMPORARY LIST
        ListTemporary = Vec::new();
    }
    // END FOR LOOP

    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("WITHIN FUNCTION:  END FUNCTION #7 - DATA OBJECTS CREATE");

    // RETURN VARIABLES TO PROGRAM
    ListOfIndexes4LettersInEachWord

}
// END FUNCTION() #7 - DATA OBJECTS CREATE