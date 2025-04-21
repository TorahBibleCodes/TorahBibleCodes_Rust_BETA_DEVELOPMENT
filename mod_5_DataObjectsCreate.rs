// IMPORT MODULES
use indexmap::IndexMap;

/// A module for creating word-based data objects from Hebrew Bible verse texts.
/// 
/// MODULE.FUNCTION() #5 - CREATE INITIAL DATA OBJECTS FROM DS; RETURNS -> TUPLE OF (ListOfWords, ListOfNumbersOfWordsEachVerse, DictionaryOfWordsEachVerse, DictionaryOfWordsTotal // LW, LNWEV, DWV, DWT)
///
/// This module provides functionality to process a verse map into word-based data structures,
/// including a list of words (`ListOfWords`), a list of word counts per verse (`ListOfNumbersOfWordsEachVerse`),
/// an indexed map of words with a 4-integer tuple key for word positions (`DWV`), and an indexed map of words with a 5-integer tuple key (`DWT`) for word positions.
/// 
/// This function processes an input `IndexMap` that maps verse references to text strings, and creates word-based data objects from a verse map, including word lists and indexed maps. It
/// iterates through each verse, splits the text into words, and constructs the data structures to be returned.
/// 
/// # Arguments
/// * `VerseMap_DS` - An `IndexMap<(u32, u32, u32), String>` mapping verse references
///   `(Book#, Chapter#, Verse#)` to text strings.
/// 
/// # Returns
/// A tuple containing:
/// - `ListOfWords`: A `Vec<String>` (i.e. List of Strings) containing all words of the text from the verses as one complete sequence of words.
/// - `ListOfNumbersOfWordsEachVerse`: A `Vec<u32>` containing the number of words in each verse.
/// - `DWV`: An `IndexMap<(u32, u32, u32, u32), String>` mapping 4-integer tuple keys `(Book#, Chapter#, Verse#, WordPosition#InVerse)` to all individual words in the text as one complete sequence of words.
/// - `DWT`: An `IndexMap<(u32, u32, u32, u32, u32), String>` mapping 5-integer tuple keys `(Book#, Chapter#, Verse#, WordPosition#InVerse, WordPosition#Total)` to all individual words in the text as one complete sequence of words.
/// 
/// Example input `VerseMap_DS`:
/// - `(1, 1, 1)`: "בראשית ברא אלהים את השמים ואת הארץ"
/// - `(1, 1, 2)`: "והארץ היתה תהו ובהו וחשך על פני תהום ורוח אלהים מרחפת על פני המים"
/// - `(1, 1, 3)`: "ויאמר אלהים יהי אור ויהי אור"
/// 
/// Returns a tuple contains:
/// - `ListOfWords`: `[ ... ]`
/// - `ListOfNumbersOfWordsEachVerse`: `[ ... ]`
/// - `DWV`: `{ ... }`
/// - `DWT`: `{ ... }`
/// 
/// # Examples
/// ```
/// // CALL MODULE.FUNCTION() #5 - CREATE INITIAL DATA OBJECTS FROM DS; RETURNS -> TUPLE OF (ListOfWords, ListOfNumbersOfWordsEachVerse, DictionaryOfWordsEachVerse, DictionaryOfWordsTotal // LW, LNWEV, DWV, DWT)
/// let (ListOfWords, ListOfNumbersOfWordsEachVerse, DWV, DWT) = mod_5_DataObjectsCreate::fn_DataObjectsCreate(VerseMap_DS);
/// ```
/// 
/// # Panics
/// This function does not panic under normal circumstances. It assumes valid input data
/// and processes text strings using whitespace splitting to identify words.
/// 
/// MODULE.FUNCTION() #5 - CREATE INITIAL DATA OBJECTS FROM DS; RETURNS -> TUPLE OF (ListOfWords, ListOfNumbersOfWordsEachVerse, DictionaryOfWordsEachVerse, DictionaryOfWordsTotal // LW, LNWEV, DWV, DWT)
/// 
// BEGIN FUNCTION() #5 - DATA OBJECTS CREATE WORDS FROM VERSES
pub fn fn_DataObjectsCreate(VerseMap_DS: IndexMap<(u32, u32, u32), String>) -> (
    Vec<String>,                                      // ListOfWords: All words in selected text
    Vec<u32>,                                       // ListOfNumbersOfWordsEachVerse: Count of words per verse
    IndexMap<(u32, u32, u32, u32), String>,           // DWV: Dictionary of words in verse with 4-digit tuple key
    IndexMap<(u32, u32, u32, u32, u32), String>,      // DWT: Dictionary of words total with 5-digit tuple key
) {
    
    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("WITHIN FUNCTION:  BEGIN FUNCTION #5 - DATA OBJECTS CREATE WORDS FROM VERSES");

    // DECLARE VARIABLES
    let mut ListOfWords: Vec<String> = Vec::new();  // CREATE EMPTY LIST TO CONTAIN LIST OF WORDS FROM SELECTED TEXT(S)
    let mut ListOfNumbersOfWordsEachVerse: Vec<u32> = Vec::new(); // COUNTS NUMBER OF WORDS IN EACH VERSE
    let mut DWV: IndexMap<(u32, u32, u32, u32), String> = IndexMap::new();  // DictionaryOfWordsInVerse
    let mut DWT: IndexMap<(u32, u32, u32, u32, u32), String> = IndexMap::new();  // DictionaryOfWordsTotal
    let mut TotalWordCounter: u32 = 1;

    // BEGIN FOR LOOP
    // FOR EACH KEY:VALUE PAIR IN STRING/VERSE IN MAP "DS"...
    for (key, each) in &VerseMap_DS {  // EACH VERSE
        
        // COUNT LENGTH OF STRING, i.e. HOW MANY LETTERS IN EACH STRING
        // let LengthOfString: usize = each.chars().count();  // EACH VERSE
        
        // SPLIT STRING INTO LIST OF WORDS
        let ListOfWordsEachVerse: Vec<String> = each
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
            
        // COUNT NUMBER OF WORDS IN VERSE
        let NumberOfWordsEachVerse: usize = ListOfWordsEachVerse.len();
        
        let mut VerseWordCounter: u32 = 1;
        
        // FOR LOOP
        for EachWord in &ListOfWordsEachVerse {
            // EXPAND TUPLE KEY
            let keyWV = (key.0, key.1, key.2, VerseWordCounter);
            
            // CREATE DICTIONARY OF WORDS IN VERSE - 4-DIGIT TUPLE-KEY
            DWV.insert(keyWV, EachWord.clone());
            
            // EXPAND TUPLE KEY
            let keyWT = (key.0, key.1, key.2, VerseWordCounter, TotalWordCounter);
            
            // CREATE DICTIONARY OF WORDS TOTAL - 5-DIGIT TUPLE-KEY
            DWT.insert(keyWT, EachWord.clone());
            
            // INCREMENT
            VerseWordCounter += 1;
            TotalWordCounter += 1;
        }
        // END FOR LOOP
        
        // ADD NUMBER OF WORDS IN EACH VERSE TO LIST OF NUMBERS
        ListOfNumbersOfWordsEachVerse.push(NumberOfWordsEachVerse as u32); // usize
        
        // CREATE LIST OF WORDS FOR ALL WORDS IN THE SELECTED TEXT
        ListOfWords.extend(ListOfWordsEachVerse);  // EXTEND THE LIST WITH ALL WORDS FROM THIS VERSE
    }
    // END FOR LOOP

    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("WITHIN FUNCTION:  END FUNCTION #5 - DATA OBJECTS CREATE WORDS FROM VERSES");
    
    // RETURN VARIABLES TO PROGRAM
    // RETURN TUPLE OF (LIST OF WORDS, LIST OF NUMBERS OF WORDS EACH VERSE, DICTIONARY OF WORDS IN VERSE, DICTIONARY OF WORDS TOTAL)
    return (ListOfWords, ListOfNumbersOfWordsEachVerse, DWV, DWT)

}
// END FUNCTION() #5 - DATA OBJECTS CREATE WORDS FROM VERSES