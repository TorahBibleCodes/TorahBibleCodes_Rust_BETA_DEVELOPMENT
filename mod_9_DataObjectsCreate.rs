// IMPORT MODULES
use indexmap::IndexMap;

/// A module for creating a data structure that maps word position indices to DWT keys in a Hebrew text sequence.
///
/// MODULE.FUNCTION() #9 - DATA OBJECTS CREATE; RETURNS -> DWTK == DICT OF DWT KEYS
///
/// This module provides functionality to process a dictionary of Hebrew word positions (`DWT`) and generate a new dictionary (`DWTK`)
/// that maps sequential word position indices to the 5-tuple keys of `DWT`. It creates a data structure that tracks the position index
/// of each word in the text sequence, associating each index with its corresponding `DWT` key (Book, Chapter, Verse, WordPosition, TotalPosition).
///
/// This function iterates through the keys of the provided `DWT` dictionary, assigns a sequential position index (starting from 1) to each key,
/// and stores the key in `DWTK` with the index as the key. The function ensures that word positions are tracked in the order of `DWT` keys.
///
/// # Arguments
/// * `DWT` - An `IndexMap<(u32, u32, u32, u32, u32), String>` mapping 5-tuple keys
///   `(Book#, Chapter#, Verse#, WordPosition#InVerse, TotalPosition#InText)` to Hebrew words.
///
/// # Returns
/// An `IndexMap<u32, (u32, u32, u32, u32, u32)>` where:
/// - Keys are sequential word position indices (starting from 1) as `u32`.
/// - Values are the 5-tuple keys from `DWT` as `(u32, u32, u32, u32, u32)`.
///
/// Example input `DWT`:
/// - `(1, 1, 1, 1, 1)`: "בראשית"
/// - `(1, 1, 1, 2, 2)`: "ברא"
/// - `(1, 1, 1, 3, 3)`: "אלהים"
///
/// Example output `DWTK`:
/// - `1`: `(1, 1, 1, 1, 1)`
/// - `2`: `(1, 1, 1, 2, 2)`
/// - `3`: `(1, 1, 1, 3, 3)`
///
/// # Examples
/// ```
/// // CALL MODULE.FUNCTION() #9 - DATA OBJECTS CREATE; RETURNS -> DWTK == DICT OF DWT KEYS
/// let DWTK = mod_9_DataObjectsCreate::fn_DataObjectsCreate(DWT);
/// ```
///
/// # Panics
/// This function does not panic under normal circumstances. It assumes a valid `IndexMap` input and processes its keys sequentially.
///
/// MODULE.FUNCTION() #9 - DATA OBJECTS CREATE; RETURNS -> DWTK == DICT OF DWT KEYS
/// 
// BEGIN FUNCTION() #9 - DATA OBJECTS CREATE
pub fn fn_DataObjectsCreate(DWT: IndexMap<(u32, u32, u32, u32, u32), String>) -> IndexMap<u32, (u32, u32, u32, u32, u32)> {
    
    // TEST PRINT OUTPUT
    println!("\n"); // PRINT SPACE
    println!("WITHIN FUNCTION:  BEGIN FUNCTION #9 - DATA OBJECTS CREATE");
    
    // DECLARE VARIABLES 
    let mut WordCounter = 1;
    let mut DWTK: IndexMap<u32, (u32, u32, u32, u32, u32)> = IndexMap::new();

    // BEGIN FOR LOOP
    // FOR EACH KEY IN DICTIONARY "DWT"...
    for EachKey in DWT.keys() { // EACH WORD
        
        // ADD EACH CURRENT TOTAL-WORD-COUNT-VALUE-INDEX-POSITION-IN-DWT (I.E. EACH KEY) TO BE VALUE OF DWTK WITH THE KEY OF DWTK THE SAME VALUE
        DWTK.insert(WordCounter, *EachKey);

        // INCREMENT WORD COUNTER
        WordCounter += 1;
    }
    // END FOR LOOP

    // TEST PRINT OUTPUT
    println!("\n"); // PRINT SPACE
    println!("WITHIN FUNCTION:  END FUNCTION #9 - DATA OBJECTS CREATE");

    // RETURN VARIABLES TO PROGRAM
    DWTK
}
// END FUNCTION() #9 - DATA OBJECTS CREATE