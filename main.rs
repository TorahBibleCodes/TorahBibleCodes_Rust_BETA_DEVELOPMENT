#![allow(non_snake_case, unused_variables, unused_assignments)]

// IMPORT MODULES
// use std::collections::HashMap;
use indexmap::IndexMap;
mod mod_0_GetUserInput_CodexToSearch; // IMPORT MODULE.FUNCTION() #0 - GET USER INPUT; CHOOSE CODEX TO SEARCH// RETURNS INTEGER
mod mod_1_GetUserInput_TextDetails; // IMPORT MODULE.FUNCTION() #1 - GET NAME OF CODEX // RETURNS STRING
mod mod_2_OpenJSONFile; // IMPORT MODULE.FUNCTION() #2 - CONVERT JSON TEXT FILES TO HASHMAPS FOR DICTIONARY OF VERSES (NO SPACES) & DICTIONARY OF VERSES (WITH SPACES)

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
        // GET USER INPUT
        // CALL MODULE.FUNCTION() #0 - GET USER INPUT 0 - CHOOSE CODEX TO SEARCH
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

        // CALL MODULE.FUNCTION() #1 - GET TEXT DETAILS - 
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
            
            // CALL MODULE.FUNCTION() #2A - OPEN JSON FILE 
            let (FilePath_D, FilePath_DS) = mod_2_OpenJSONFile::fn_OpenJSONFile(NumberOfCodex); //

            // TEST PRINT OUTPUT
            println!("FilePath_D: {}", FilePath_D );
            println!("FilePath_DS: {}", FilePath_DS );

            // CALL MODULE.FUNCTION() #2B - LOAD AND PARSE JSON 
            let verse_map_D: IndexMap<(u32, u32, u32), String> = mod_2_OpenJSONFile::fn_LoadAndParseJSON(FilePath_D); //

            // CALL MODULE.FUNCTION() #2B - LOAD AND PARSE JSON 
            let verse_map_DS: IndexMap<(u32, u32, u32), String> = mod_2_OpenJSONFile::fn_LoadAndParseJSON(FilePath_DS); //

            // TEST PRINT OUTPUT
            for ((book, chapter, verse), text) in &verse_map_D {
                println!("({},{},{}): {}", book, chapter, verse, text);
            }
            
        }
        // END IF / ELSE
    } 
    // END WHILE LOOP

}
// END CALL MAIN FUNCTION
// END MAIN PROGRAM

