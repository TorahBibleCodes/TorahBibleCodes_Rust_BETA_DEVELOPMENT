// IMPORT MODULES
use indexmap::IndexMap;
use crate::mod_cls_99_LetterObject::LetterObject;

/// A module for creating letter-based data objects from Hebrew Bible verse texts.
/// 
/// MODULE.FUNCTION() #4 - DATA OBJECTS CREATE LETTERS FROM VERSES - CREATE INITIAL DATA OBJECTS FROM D // RETURNS -> TUPLE OF (S, L, DL, D5, DLO)
/// 
/// This module provides functionality to process a verse map into various initial data objects (data structures),
/// including a string sequence (`S`), a list of letters (`L`), an indexed map of letters with a 4-integer tuple key (`DL`), an indexed map of letters with a 5-integer tuple key (`D5`),
/// and an indexed map of custom class letter objects (`DLO`).
/// 
/// This function processes an input `IndexMap` mapping verse references to text strings,
/// generating a string sequence (`S`) of letters, a vector of letters (`L`), two indexed maps with
/// 4-integer tuple keys (`DL`) and 5-integer tuple keys (`D5`) for letter positions, as well as an indexed map of `LetterObject` instances. 
/// It iterates through each verse, processes individual letters, and constructs the data structures to return.
/// 
/// # Arguments
/// * `VerseMap_D` - An `IndexMap<(u32, u32, u32), String>` mapping verse references `(Book#, Chapter#, Verse#)` to text strings, i.e. individual Hebrew text characters.
/// 
/// # Returns
/// A tuple containing:
/// - `S`: A `String` sequence of all letters in the text from the verses as one complete sequence of letters.
/// - `L`: A `Vec<char>` containing all letters in the text from the verses as one complete sequence of letters.
/// - `DL`: An `IndexMap<(u32, u32, u32, u32), char>` mapping 4-integer tuple keys `(Book#, Chapter#, Verse#, LetterPosition#InVerse)` to all individual letters in the text from the verses as one complete sequence of letters.
/// - `D5`: An `IndexMap<(u32, u32, u32, u32, u32), char>` mapping 5-integer tuple keys `(Book#, Chapter#, Verse#, LetterPosition#InVerse, LetterPosition#InText)` to all individual letters in the text from the verses as one complete sequence of letters.
/// - `DictOfLetterObjects`: An `IndexMap<u32, LetterObject>` mapping letter position index numbers (`n`) to `LetterObject` instances for all individual letters in the text from the verses as one complete sequence of letters.
/// 
/// Example input `VerseMap_D`:
/// - `(1, 1, 1)`: `'בראשיתבראאלהיםאתהשמיםואתהארץ'`
/// - `(1, 1, 2)`: `'והארץהיתהתהוובהווחשךעלפניתהוםורוחאלהיםמרחפתעלפניהמים'`
/// - `(1, 1, 3)`: `'ויאמראלהיםיהיאורויהיאור'`
/// 
/// Returns 5-integer tuple of:
/// - `S`: `" ... "`
/// - `L`: `[ ... ]`
/// - `DL`: `{ ... }`
/// - `D5`: `{ ... }`
/// - `DictOfLetterObjects`: `{ ... }`
/// 
/// # Examples
/// ```
/// // CALL MODULE.FUNCTION() #4 - DATA OBJECTS CREATE LETTERS FROM VERSES - CREATE INITIAL DATA OBJECTS FROM D // RETURNS -> TUPLE OF (S, L, DL, D5, DLO)
/// let (S, L, DL, D5, DLO) = mod_4_DataObjectsCreate::fn_DataObjectsCreate(VerseMap_D);
/// ```
/// 
/// # Panics
/// This function does not panic under normal circumstances. It assumes valid input data
/// and relies on the `LetterObject::fn_ConstructLO` method to handle letter object creation.
/// 
/// MODULE.FUNCTION() #4 - DATA OBJECTS CREATE LETTERS FROM VERSES - CREATE INITIAL DATA OBJECTS FROM D // RETURNS -> TUPLE OF (S, L, DL, D5, DLO)
///
// BEGIN FUNCTION() #4 - DATA OBJECTS CREATE LETTERS FROM VERSES - CREATE INITIAL DATA OBJECTS FROM D // RETURNS (S, L, DL, D5, DLO)
pub fn fn_DataObjectsCreate(VerseMap_D: IndexMap<(u32, u32, u32), String>) -> ( 

    String,                                     // S: STRING SEQUENCE OF LETTERS
    Vec<char>,                                  // L: LIST OF LETTERS
    IndexMap<(u32, u32, u32, u32), char>,       // DL: DICTIONARY OF LETTERS WITH 4-INTEGER TUPLE KEY
    IndexMap<(u32, u32, u32, u32, u32), char>,  // D5: DICTIONARY OF LETTERS WITH 5-INTEGER TUPLE KEY
    IndexMap<u32, LetterObject>,                // DictOfLetterObjects: DICT OF LETTER (CLASS) OBJECTS --> DLO
) {
    
    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("WITHIN FUNCTION:  BEGIN FUNCTION #4 - DATA OBJECTS CREATE LETTERS FROM VERSES");

    // DECLARE VARIABLES
    let mut DL: IndexMap<(u32, u32, u32, u32), char> = IndexMap::new();
    let mut D5: IndexMap<(u32, u32, u32, u32, u32), char> = IndexMap::new();
    let mut L: Vec<char> = Vec::new();
    let mut DictOfLetterObjects: IndexMap<u32, LetterObject> = IndexMap::new();
    let mut VerseLetterCounter: u32 = 1;
    let mut TotalLetterCounter: u32 = 1;

    // ITERATE THROUGH EACH KEY-VALUE PAIR IN THE INPUT INDEXMAP
    for (key, each) in &VerseMap_D {

        // COUNT LENGTH OF STRING
        // let LengthOfString = each.len();

        // PROCESS EACH LETTER IN THE TEXT STRING OF VERSE
        for letter in each.chars() {

            // CREATE LIST OF LETTERS
            L.push(letter);

            // EXPAND TUPLE 3-INTEGER TUPLE KEY TO 4 INTEGERS
            let key4: (u32, u32, u32, u32) = (key.0, key.1, key.2, VerseLetterCounter);

            // CREATE DICTIONARY OF LETTERS WITH 4-INTEGER TUPLE KEY
            DL.insert(key4, letter);

            // EXPAND TUPLE 4-INTEGER TUPLE KEY TO 5-INTEGER TUPLE KEY
            let key5: (u32, u32, u32, u32, u32) = (key.0, key.1, key.2, VerseLetterCounter, TotalLetterCounter);

            // CREATE DICTIONARY OF LETTERS WITH 5-INTEGER TUPLE KEY
            D5.insert(key5, letter);

            // INITIALIZE/CREATE INSTANCE OF CLASS LETTER OBJECT
            let lo = LetterObject::fn_ConstructLO(
                letter,
                TotalLetterCounter,
                key5,
                key4,
                *key,
            );

            // CREATE KEY IN DICT OF LETTER OBJECTS + ADD INSTANCE OF EACH LETTER OBJECT
            DictOfLetterObjects.insert(TotalLetterCounter, lo);

            // INCREMENT LETTER COUNTERS
            TotalLetterCounter += 1;
            VerseLetterCounter += 1;
        }

        // RESET VERSE LETTER COUNTER
        VerseLetterCounter = 1;
    }

    // CREATE STRING-SEQUENCE OF LETTERS FROM LIST OF LETTERS
    let S: String = L.iter().collect();

    // TEST PRINT OUTPUT
    // println!("S: {}", S );

    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("WITHIN FUNCTION:  END FUNCTION #4 - DATA OBJECTS CREATE LETTERS FROM VERSES");

    // RETURN VALUES
    (S, L, DL, D5, DictOfLetterObjects)
    
}
// END FUNCTION() #4 - DATA OBJECTS CREATE LETTERS FROM VERSES - CREATE INITIAL DATA OBJECTS FROM D // RETURNS (S, L, DL, D5, DLO)