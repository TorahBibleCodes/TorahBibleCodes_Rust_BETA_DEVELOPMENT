#![allow(non_snake_case, unused_variables, unused_assignments)]

// IMPORT MODULES
use indexmap::IndexMap;
mod mod_cls_99_LetterObject; // IMPORT MODULE.CLASS() # -
mod mod_0_GetUserInput_CodexToSearch; // IMPORT MODULE.FUNCTION() #0 - GET USER INPUT; CHOOSE CODEX TO SEARCH// RETURNS INTEGER
mod mod_1_GetUserInput_TextDetails; // IMPORT MODULE.FUNCTION() #1 - GET NAME OF CODEX // RETURNS STRING
mod mod_2_GetFilePaths; // IMPORT MODULE.FUNCTION() #2 - GET FILE PATHS // RETURNS TUPLE OF STRINGS FOR EACH DICTIONARY OF VERSES (NO SPACES: D) & DICTIONARY OF VERSES (WITH SPACES: DS)
mod mod_3_LoadAndParseJSON; // IMPORT MODULE.FUNCTION() #3 - CONVERT JSON TEXT FILES TO IndexMap FOR DICTIONARY OF VERSES (NO SPACES: D) & DICTIONARY OF VERSES (WITH SPACES: DS)
mod mod_4_DataObjectsCreate; // IMPORT MODULE.FUNCTION() #4 - CREATE INITIAL DATA OBJECTS FROM D // RETURNS (S, L, DL, D5, DLO)
mod mod_5_DataObjectsCreate; // IMPORT MODULE.FUNCTION() #5 - CREATE INITIAL DATA OBJECTS FROM DS // RETURNS ListOfWords, ListOfNumbersOfWordsEachVerse, DictionaryOfWordsEachVerse, DictionaryOfWordsTotal // LW, LNWEV, DWV, DWT
mod mod_6_CalculateLetterPercentages; // IMPORT MODULE.FUNCTION() #6 - CALCULATE LETTER PERCENTAGES
mod mod_7_DataObjectsCreate; // IMPORT MODULE.FUNCTION() #7 - CREATE SECONDARY DATA OBJECTS FROM LIST OF WORDS (LW) - RETURNS LIST OF LISTS OF INDICES (INDEXES) FOR LETTERS IN EACH WORD
mod mod_8_DataObjectsCreate; // IMPORT MODULE.FUNCTION() #8
mod mod_9_DataObjectsCreate; // IMPORT MODULE.FUNCTION() #9
mod mod_10_GetNumberValues4Letters; // IMPORT MODULE.FUNCTION() #10
mod mod_11_AddGematriaNumberValuesToLetterObjects; // IMPORT MODULE.FUNCTION() #11

// BEGIN MAIN PROGRAM
// BEGIN CALL MAIN FUNCTION
fn main() {
    
    // DECLARE VARIABLES
    let mut IsGameOver: bool = false; // FOR THE INFINITE WHILE LOOP TO KEEP THE PROGRAM RUNNING
    let mut IsTextSelected: bool = false; // TO ONLY ALLOW ONE TEXT PER GAME TO BE SELECTED

    // BEGIN 
    // BEGIN WHILE LOOP FOR INFINITE GAME WHILE LOOP
    while IsGameOver == false && IsTextSelected == false {
        
        // TEST PRINT OUTPUT
        println!("\n");  // PRINT SPACE
        println!("This will loop forever unless you break.");

        // BEGIN CALL MODULES.FUNCTIONS()
        // CALL MODULE.FUNCTION() #0 - GET USER INPUT 0 - CHOOSE CODEX TO SEARCH
        // RETURNS -> NUMBER OF CODEX
        // NumberOfCodexChosen = mod_0_GetUserInput_CodexToSearch.fn_GetUserInput() // PYTHON
        let NumberOfCodex: u32 = mod_0_GetUserInput_CodexToSearch::fn_GetUserInput(); // 8-bit unsigned integer (0 to 255)

        // TEST PRINT OUTPUT
        // println!("\n");  // PRINT SPACE
        // println!("You have chosen codex number #: {}.", NumberOfCodexChosen);

        // BEGIN IF / ELSE
        if NumberOfCodex == 0 {

            // TEST PRINT OUTPUT
            println!("NumberOfCodex: {}", NumberOfCodex );

            // CHANGE STATUS TO GAME OVER: TRUE
            IsGameOver = true;
            IsTextSelected = false;
            
            // TEST PRINT OUTPUT
            println!("IsTextSelected: {}", IsTextSelected ); // false
            println!("GAME OVER: {}", IsGameOver ); // true

            // MANUALLY BREAK OUT OF WHILE LOOP
            break;
            
        }
        // END IF / ELSE

        // CALL MODULE.FUNCTION() #1 - GET USER INPUT - TEXT DETAILS -
        // RETURNS -> (1. NAME OF CODEX; 2. NUMBER OF TEXT CHOSEN) 
        let (NameOfCodex, NumberOfTextChosen) =  mod_1_GetUserInput_TextDetails::fn_GetUserInput(NumberOfCodex); //
        
        // TEST PRINT OUTPUT
        // println!("\n");  // PRINT SPACE
        // println!("NameOfCodex: {}.", NameOfCodex);
        // println!("NumberOfTextChosen: {}.", NumberOfTextChosen);

        // BEGIN IF / ELSE
        if NumberOfTextChosen == 0 {
 
            // TEST PRINT OUTPUT
            println!("NumberOfTextChosen: {}", NumberOfTextChosen );

            // CHANGE STATUS TO GAME OVER: TRUE
            IsGameOver = true;
            IsTextSelected = false;
            
            // TEST PRINT OUTPUT
            println!("IsTextSelected: {}", IsTextSelected ); // false
            println!("GAME OVER: {}", IsGameOver ); // true
            
            // MANUALLY BREAK OUT OF WHILE LOOP
            break;
        
        }
        // END IF / ELSE

        // BEGIN IF / ELSE
        if NumberOfTextChosen != 0 {
 
            // TEST PRINT OUTPUT
            println!("NumberOfTextChosen: {}", NumberOfTextChosen );

            // CHANGE STATUS TO GAME OVER: TRUE
            // IsGameOver = false;
            IsTextSelected = true;
            
            // TEST PRINT OUTPUT
            println!("IsTextSelected: {}", IsTextSelected ); // true
            println!("GAME OVER: {}", IsGameOver ); // false
            
            // CALL MODULE.FUNCTION() #2 - OPEN JSON FILE
            // RETURNS -> FILE PATHS 
            let (FilePath_D, FilePath_DS) = mod_2_GetFilePaths::fn_GetFilePaths(NumberOfCodex);

            // TEST PRINT OUTPUT
            println!("FilePath_D: {}", FilePath_D );
            println!("FilePath_DS: {}", FilePath_DS );

            // CALL MODULE.FUNCTION() #3 - LOAD AND PARSE JSON
            // RETURNS -> DICTIONARY OF VERSES (NO SPACES) 
            let VerseMap_D: IndexMap<(u32, u32, u32), String> = mod_3_LoadAndParseJSON::fn_LoadAndParseJSON(FilePath_D);
            
            // CALL MODULE.FUNCTION() #3 - LOAD AND PARSE JSON 
            // RETURNS -> DICTIONARY OF VERSES (WITH SPACES)
            let VerseMap_DS: IndexMap<(u32, u32, u32), String> = mod_3_LoadAndParseJSON::fn_LoadAndParseJSON(FilePath_DS);

            // TEST PRINT OUTPUT
            /*
            for ((book, chapter, verse), text) in &VerseMap_D {
                println!("({},{},{}): {}", book, chapter, verse, text);
            }
            */
            
            // CALL MODULE.FUNCTION() #4 - CREATE INITIAL DATA OBJECTS FROM LETTERS IN EACH VERSE + CREATE DICTIONARY OF CUSTOM CLASS LETTER OBJECTS (DLO)
            // RETURNS -> (1.) STRING OF LETTERS; 2.) LIST OF LETTERS; 3.) DICT OF LETTERS WITH 4-DIGIT TUPLE KEY; 4.) DICT OF LETTERS WITH 5-DIGIT TUPLE KEY; 5.) DICT OF INSTANCES OF LETTER OBJECTS)
            let (S, L, DL, D5, DLO) = mod_4_DataObjectsCreate::fn_DataObjectsCreate(VerseMap_D);
            
            // TEST PRINT OUTPUT
            // println!("S: {}; Length (Bytes): {}; Length (Unicode Characters): {}", S, S.len(), S.chars().count()); // LENGTH IN BYTES BY DEFAULT // LENGTH OF UNICODE CHARACTERS
            // println!("L: '{:?}; Length: {}'", L, L.len());
            // println!("DL: {:?}; {}", DL, DL.len()); 
            // println!("D5: {:?}; {}", D5, D5.len()); 
            // println!("DLO: {:?}; {}", DLO, DLO.len());

            // CALL MODULE.FUNCTION() #5 - CREATE INITIAL DATA OBJECTS FROM WORDS IN EACH VERSE
            // RETURNS -> (1.) LIST OF WORDS; 2.) LIST OF NUMBERS OF WORDS IN EACH VERSE; 3.) DICTIONARY OF WORDS IN THE VERSE; 4.) DICTIONARY OF WORDS TOTAL)
            let (LW, LNWEV, DWV, DWT) = mod_5_DataObjectsCreate::fn_DataObjectsCreate(VerseMap_DS); // LW, LNWEV, DWV, DWT

            // TEST PRINT OUTPUT
            // println!("ListOfWords: '{:?}; Length: {}'", ListOfWords, ListOfWords.len());
            // println!("ListOfNumbersOfWordsEachVerse: '{:?}; Length: {}'", ListOfNumbersOfWordsEachVerse, ListOfNumbersOfWordsEachVerse.len());
            // println!("DWV: {:?}; {}", DWV, DWV.len());
            // println!("DWT: {:?}; {}", DWT, DWT.len()); 

            // CALL MODULE.FUNCTION() #6 - CALCULATE LETTER PERCENTAGES
            // RETURNS -> LIST OF TUPLES OF LETTER STATISTICS
            let ListOfTuplesOfLetterStatistics:Vec<(String, u32, u32, f64, f64)> = mod_6_CalculateLetterPercentages::fn_CalculatePercentages(&S);
            
            // TEST PRINT OUTPUT
            // println!("ListOfTuplesOfLetterStatistics: {:?}; {}", ListOfTuplesOfLetterStatistics, ListOfTuplesOfLetterStatistics.len());
            /*
            for stat in ListOfTuplesOfLetterStatistics {
                println!("Letter: {}, Count: {}, Total: {}, Decimal: {:.4}, Percentage: {:.2}%", 
                    stat.0, stat.1, stat.2, stat.3, stat.4);
            } 
            */

            // CALL MODULE.FUNCTION() #7 - CREATE SECONDARY DATA OBJECTS FROM LIST OF WORDS (LW) - RETURNS LIST OF LISTS OF INDICES (INDEXES) FOR LETTERS IN EACH WORD
            let ListOfIndexes4LettersInEachWord:Vec<Vec<u32>> = mod_7_DataObjectsCreate::fn_DataObjectsCreate(LW);

            // TEST PRINT OUTPUT - LIST OF LISTS
            // println!("ListOfIndexes4LettersInEachWord: '{:?}; Length: {}'", ListOfIndexes4LettersInEachWord, ListOfIndexes4LettersInEachWord.len());

            // CALL MODULE.FUNCTION() #8 - DATA OBJECTS CREATE; RETURNS -> D5K == DICT OF D5 KEYS // DATA OBJECTS CREATE - RETURNS DICT OF D5 KEYS AS VALUES WITH 1-INDEXED KEY FOR # OF LETTERS IN TEXT
            let D5K = mod_8_DataObjectsCreate::fn_DataObjectsCreate(D5);

            // TEST PRINT OUTPUT
            // println!("{:?}", D5K);

            // CALL MODULE.FUNCTION() #9 - DATA OBJECTS CREATE; RETURNS -> DWTK == DICT OF DWT KEYS
            let DWTK = mod_9_DataObjectsCreate::fn_DataObjectsCreate(DWT);

            // TEST PRINT OUTPUT
            //println!("{:?}", DWTK);

            // CALL MODULE.FUNCTION() #10 - GET NUMBER VALUE OF EACH LETTER IN LETTER STRING
            let N = mod_10_GetNumberValues4Letters::fn_GetNumberValues(L); // ListOfNumberValues4Letter

            // TEST PRINT OUTPUT
            // println!("{:?}", N);

            // CALL MODULE.FUNCTION() #9AAA - ADD GEMATRIA NUMBER VALUES TO LETTER OBJECTS
            // RETURNS -> UPDATED DLO
            let DLO = mod_11_AddGematriaNumberValuesToLetterObjects::fn_AddGematriaNumberValuesToLetterObjects(DLO, N);

            // TEST PRINT OUTPUT
            println!("DLO: {:?}", DLO);

        }
        // END IF / ELSE
    } 
    // END WHILE LOOP

}
// END CALL MAIN FUNCTION
// END MAIN PROGRAM

