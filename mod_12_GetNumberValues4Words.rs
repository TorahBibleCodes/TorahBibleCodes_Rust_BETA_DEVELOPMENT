// IMPORT MODULES
use crate::mod_10_GetNumberValues4Letters;

// FUNCTION() #12 - GET NUMBER VALUES FOR EACH WORDSTRING IN LIST OF WORDS
#[allow(unused_parens)]
pub fn fn_GetNumberValues(ListOfWords: &Vec<String>) -> Vec<(u32, Vec<u32>, u32)> {
    
    /*
    MODULE.FUNCTION() #12 - GET NUMBER VALUE OF EACH LETTER IN WORD STRING  RETURNS ListOfNumberValues4Words
    */

    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("WITHIN FUNCTION:  BEGIN FUNCTION #12 - GET NUMBER VALUES FOR WORDS");

    // DECLARE VARIABLES
    let mut ListOfNumberValues4Words: Vec<(u32, Vec<u32>, u32)> = Vec::new(); // CREATE EMPTY LIST TO STORE VALUES
    let mut WordCounter: u32 = 1; // INITIALIZE WORD COUNTER

    // BEGIN FOR LOOP
    // FOR EACH ELEMENT IN SEQUENCE S, L, etc.??
    for EachWord in ListOfWords.iter() {

        // TEST PRINT OUTPUT
        // print(f"{WordCounter} EachWord : {EachWord[::-1]}")
        // println!("{} EachWord : {}", WordCounter, EachWord.chars().rev().collect::<String>());

        // CALL FUNCTION - 
        let chars_vec: Vec<char> = EachWord.chars().collect();
        let NumberValuesForEachWord = mod_10_GetNumberValues4Letters::fn_GetNumberValues(&chars_vec);

        // TEST PRINT OUTPUT
        // print("\n")  // PRINT SPACE
        // print(NumberValuesForEachWord)
        // println!("\n");  // PRINT SPACE
        // println!("{:?}", NumberValuesForEachWord);

        // TEST PRINT OUTPUT
        // print("\n")  // PRINT SPACE
        // print(type(NumberValuesForEachWord))
        // println!("\n");  // PRINT SPACE
        // println!("type(NumberValuesForEachWord)");

        // SUM TOTAL NUMBER VALUE FOR EACH WORD
        let TotalNumberValueForEachWord: u32 = NumberValuesForEachWord.iter().sum();

        // TEST PRINT OUTPUT
        // print("\n")  // PRINT SPACE
        // print(TotalNumberValueForEachWord)
        // println!("\n");  // PRINT SPACE
        // println!("{}", TotalNumberValueForEachWord);

        // TEST PRINT OUTPUT
        // print("\n")  // PRINT SPACE
        // print(type(TotalNumberValueForEachWord))
        // println!("\n");  // PRINT SPACE
        // println!("type(TotalNumberValueForEachWord)");

        // DECLARE TEMP TUPLE VARIABLE
        let t = (WordCounter, NumberValuesForEachWord, TotalNumberValueForEachWord);

        // APPEND TUPLE TO LIST OF NUMBER VALUES 4 WORDS
        ListOfNumberValues4Words.push(t);

        // INCREMENT WORD COUNTER
        WordCounter += 1;
    }
    // END FOR LOOP

    // TEST PRINT OUTPUT
    // print("\n")  // PRINT SPACE
    // print(ListOfNumberValues4Words)
    // println!("\n");  // PRINT SPACE
    // println!("{:?}", ListOfNumberValues4Words);

    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("WITHIN FUNCTION:  END FUNCTION #12 - GET NUMBER VALUES FOR WORDS");

    // RETURN VARIABLES TO PROGRAM
    return ListOfNumberValues4Words; // LIST OF TUPLES
}

// END FUNCTION() #12 - GET NUMBER VALUES FOR EACH WORDSTRING IN LIST OF WORDS