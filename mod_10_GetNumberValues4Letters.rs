// IMPORT MODULES
/// A module for calculating numerical values for Hebrew letters based on gematria.
///
/// MODULE.FUNCTION() #10 - GET NUMBER VALUE OF EACH LETTER IN LETTER STRING; RETURNS -> ListOfNumberValues4Letters
///
/// This module provides functionality to process a sequence of Hebrew letters and compute their numerical values
/// according to Hebrew gematria (e.g., א=1, ב=2, ..., ת=400). It handles regular and final forms (e.g., כ/ך=20),
/// spaces (0), and em-dashes (0), returning a vector of sums where each sum represents the value of a single letter.
///
/// This function iterates through the provided sequence of letters, assigns a numerical value to each letter based
/// on its gematria value, and appends the sum (value of one letter) to the result vector. It handles special cases
/// like spaces and em-dashes by assigning them a value of 0. The function ensures accurate mapping for all Hebrew letters.
///
/// # Arguments
/// * `SequenceOfLetters` - A `Vec<char>` containing the sequence of Hebrew letters to process.
///
/// # Returns
/// A `Vec<u32>` containing the numerical values (sums) for each letter in the input sequence.
///
/// Example input `SequenceOfLetters`:
/// - `['ב', 'ר', 'א']`
///
/// Example output `ListOfNumberValues4Letters`:
/// - `[2, 200, 1]`
///   - 'ב' = 2
///   - 'ר' = 200
///   - 'א' = 1
///
/// # Examples
/// ```
/// // CALL MODULE.FUNCTION() #10 - GET NUMBER VALUE OF EACH LETTER IN LETTER STRING
/// let ListOfNumberValues4Letters = mod_9A_GetNumberValues::fn_GetNumberValues(SequenceOfLetters);
/// ```
///
/// # Panics
/// This function does not panic under normal circumstances. It assumes valid input (a vector of characters) and processes
/// each character sequentially, assigning a value of 0 to unrecognized characters (e.g., spaces, em-dashes).
///
/// MODULE.FUNCTION() #10 - GET NUMBER VALUE OF EACH LETTER IN LETTER STRING; RETURNS -> ListOfNumberValues4Letters
/// 
// BEGIN FUNCTION() #10 - GET NUMBER VALUES FOR LETTERS
pub fn fn_GetNumberValues(SequenceOfLetters: Vec<char>) -> Vec<u32> {
    
    // TEST PRINT OUTPUT
    // println!("\n"); // PRINT SPACE
    // println!("WITHIN FUNCTION:  BEGIN FUNCTION #10 - GET NUMBER VALUES FOR LETTERS"); // COMPUTATION INTENSIVE
    
    // TEST PRINT OUTPUT
    // println!("\n"); // PRINT SPACE
    // println!("{:?}", SequenceOfLetters);

    // DECLARE VARIABLES
    // CREATE EMPTY LISTS TO STORE VALUES
    let mut ListOfNumberValues4Letters: Vec<u32> = Vec::new();
    let mut ListTemp: Vec<u32> = Vec::new(); // TEMPORARY LIST
    let mut ListSum: u32; // TEMPORARY SUM

    // BEGIN FOR LOOP
    // FOR EACH ELEMENT IN SEQUENCE S, L, etc.
    for each in SequenceOfLetters {
        
        // FOR EACH LETTER IN WORD SEQUENCE // STRING S
        let value = match each {
            'א' => 1,
            'ב' => 2,
            'ג' => 3,
            'ד' => 4,
            'ה' => 5,
            'ו' => 6,
            'ז' => 7,
            'ח' => 8,
            'ט' => 9,
            'י' => 10,
            'כ' | 'ך' => 20,
            'ל' => 30,
            'מ' | 'ם' => 40,
            'נ' | 'ן' => 50,
            'ס' => 60,
            'ע' => 70,
            'פ' | 'ף' => 80,
            'צ' | 'ץ' => 90,
            'ק' => 100,
            'ר' => 200,
            'ש' => 300,
            'ת' => 400,
            // DEAL WITH POTENTIAL SPACES IN THE ELS SEARCH TERMS
            ' ' => 0,
            // TEST DEVELOPMENT
            // DEAL WITH POTENTIAL EM-DASH IN THE MAM CODEX AFTER PARSING IN THE PROPHETS SECTION (JOSHUA) WORD (1-INDEXED) #8362 AND #8363 == EM DASH "\u2014"
            '—' => 0,
            _ => 0, // DEFAULT FOR UNRECOGNIZED CHARACTERS
        };

        // APPEND VALUE TO THE TEMPORARY LIST
        ListTemp.push(value);

        // TEST PRINT OUTPUT
        // println!("\n"); // PRINT SPACE
        // println!("value = {}", value);

        // SUM THE VALUES IN TEMPORARY LIST
        ListSum = ListTemp.iter().sum();

        // APPEND THIS VALUE OF THE SUM TO THE MAIN LIST TO RETURN LATER
        ListOfNumberValues4Letters.push(ListSum);

        // RESET ListTemp
        ListTemp = Vec::new();

        // RESET ListSum
        ListSum = 0;

        // TEST PRINT OUTPUT
        // println!("\n"); // PRINT SPACE
        // println!("{:?}", ListOfNumberValues4Letters);
    }
    // END FOR LOOP

    // TEST PRINT OUTPUT
    // println!("\n"); // PRINT SPACE
    // println!("WITHIN FUNCTION:  END FUNCTION #10 - GET NUMBER VALUES FOR LETTERS"); // COMPUTATION INTENSIVE

    // RETURN VARIABLES TO PROGRAM
    ListOfNumberValues4Letters
}
// END FUNCTION() #10 - GET NUMBER VALUES FOR LETTERS