// IMPORT MODULES
use indexmap::IndexMap;

/// A module for creating a data structure that maps letter position indices to D5 keys in a Hebrew text sequence.
///
/// MODULE.FUNCTION() #8 - DATA OBJECTS CREATE; RETURNS -> D5K == DICT OF D5 KEYS
///
/// This module provides functionality to process a dictionary of Hebrew letter positions (`D5`) and generate a new dictionary (`D5K`)
/// that maps sequential letter position indices to the 5-tuple keys of `D5`. It creates a data structure that tracks the position index
/// of each letter in the text sequence, associating each index with its corresponding `D5` key (Book, Chapter, Verse, LetterPosition, TotalPosition).
///
/// This function iterates through the keys of the provided `D5` dictionary, assigns a sequential position index (starting from 1) to each key,
/// and stores the key in `D5K` with the index as the key. The function ensures that letter positions are tracked in the order of `D5` keys.
///
/// # Arguments
/// * `D5` - An `IndexMap<(u32, u32, u32, u32, u32), char>` mapping 5-tuple keys
///   `(Book#, Chapter#, Verse#, LetterPosition#InVerse, TotalPosition#InText)` to Hebrew letters.
///
/// # Returns
/// An `IndexMap<u32, (u32, u32, u32, u32, u32)>` where:
/// - Keys are sequential letter position indices (starting from 1) as `u32`.
/// - Values are the 5-tuple keys from `D5` as `(u32, u32, u32, u32, u32)`.
///
/// Example input `D5`:
/// - `(1, 1, 1, 1, 1)`: 'ב'
/// - `(1, 1, 1, 2, 2)`: 'ר'
/// - `(1, 1, 1, 3, 3)`: 'א'
///
/// Example output `D5K`:
/// - `1`: `(1, 1, 1, 1, 1)`
/// - `2`: `(1, 1, 1, 2, 2)`
/// - `3`: `(1, 1, 1, 3, 3)`
///
/// # Examples
/// ```
/// // CALL MODULE.FUNCTION() #8 - DATA OBJECTS CREATE; RETURNS -> D5K == DICT OF D5 KEYS
/// let D5K = mod_8_DataObjectsCreate::fn_DataObjectsCreate(D5);
/// ```
///
/// # Panics
/// This function does not panic under normal circumstances. It assumes a valid `IndexMap` input and processes its keys sequentially.
///
/// MODULE.FUNCTION() #8 - DATA OBJECTS CREATE; RETURNS -> D5K == DICT OF D5 KEYS
/// 
// BEGIN FUNCTION() #8 - DATA OBJECTS CREATE
pub fn fn_DataObjectsCreate(D5: IndexMap<(u32, u32, u32, u32, u32), char>) -> IndexMap<u32, (u32, u32, u32, u32, u32)> {
    
    // TEST PRINT OUTPUT
    println!("\n"); // PRINT SPACE
    println!("WITHIN FUNCTION:  BEGIN FUNCTION #8 - DATA OBJECTS CREATE");
    
    // DECLARE VARIABLES 
    let mut LetterCounter = 1;
    let mut D5K: IndexMap<u32, (u32, u32, u32, u32, u32)> = IndexMap::new();

    // BEGIN FOR LOOP
    // FOR EACH KEY IN DICTIONARY "D5"...
    for EachKey in D5.keys() { // EACH VERSE
        
        // ADD EACH CURRENT TOTAL-LETTER-COUNT-VALUE-INDEX-POSITION-IN-D5 (I.E. EACH KEY) TO BE VALUE OF D5K WITH THE KEY OF D5K THE SAME VALUE
        D5K.insert(LetterCounter, *EachKey);

        // INCREMENT LETTER COUNTER
        LetterCounter += 1;
    }
    // END FOR LOOP

    // TEST PRINT OUTPUT
    println!("\n"); // PRINT SPACE
    println!("WITHIN FUNCTION:  END FUNCTION #8 - DATA OBJECTS CREATE");

    // RETURN VARIABLES TO PROGRAM
    D5K
}
// END FUNCTION() #8 - DATA OBJECTS CREATE