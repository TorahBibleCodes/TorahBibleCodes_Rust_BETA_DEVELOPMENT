// IMPORT MODULES
use std::io;

// FUNCTION () #1 - GET NAME OF CODEX // RETURNS STRING //
pub fn fn_GetUserInput(NumberOfCodex: u32) -> (String, u32) {

    /*
    // MODULE.FUNCTION() #1 - GET NAME OF CODEX
    */

    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("WITHIN FUNCTION: BEGIN FUNCTION #1");

    // DECLARE VARIABLES
    let mut NameOfCodex = String::from("");
    
    // BEGIN MATCH CASE - DEAL WITH CHOICE OF TEXT(S)
    match NumberOfCodex {

        // CASE: CODEX: 0 - NONE
        0 => { NameOfCodex.push_str("Codex: 0 - None"); },


        // PYTHON: CALL MODULE.FUNCTION() #1A - GET USER INPUT 1A - CHOOSE TEXT TO SEARCH - KOREN CODEX
        1 => { NameOfCodex.push_str("Codex: 1 - Koren"); 
    
        // DISPLAY OPTIONS TO USER
        println!("\n");  // PRINT SPACE
        println!("Please select text to search in {}:", NameOfCodex);
        println!("\n");  // PRINT SPACE
        println!("1 - Genesis - 78064 letters");
        println!("2 - Exodus - 63529 letters");
        println!("3 - Leviticus - 44790 letters");
        println!("4 - Numbers - 63530 letters");
        println!("5 - Deuteronomy - 54892 letters");
        println!("40 - Pentateuch (Torah) - 304805 letters");
        println!("0 - QUIT / EXIT");
    
        },
        
        // PYTHON: CALL MODULE.FUNCTION() #1B - GET USER INPUT 1B - CHOOSE TEXT TO SEARCH - LENINGRAD CODEX
        2 => { NameOfCodex.push_str("Codex: 2 - Leningrad"); 
    
        // DISPLAY OPTIONS TO USERT
        println!("\n");  // PRINT SPACE
        println!("Please select text to search in {}:", NameOfCodex);
        println!("\n");  // PRINT SPACE
        println!("1 - Genesis - 78069 letters");
        println!("2 - Exodus - 63531 letters");
        println!("3 - Leviticus - 44795 letters");
        println!("4 - Numbers - 63545 letters");
        println!("5 - Deuteronomy - 54910 letters");
        println!("6 - Joshua - 39807 letters");
        println!("7 - Judges - 38944 letters");
        println!("8 - I Samuel - 51354 letters");
        println!("9 - II Samuel - 42178 letters");
        println!("10 - I Kings - 50623 letters");
        println!("11 - II Kings - 47837 letters");
        println!("12 - Isaiah - 66888 letters");
        println!("13 - Jeremiah - 84912 letters");
        println!("14 - Ezekiel - 74499 letters");
        println!("15 - Hosea - 9385 letters");
        println!("16 - Joel - 3872 letters");
        println!("17 - Amos - 8033 letters");
        println!("18 - Obadiah - 1119 letters");
        println!("19 - Jonah - 2700 letters");
        println!("20 - Micah - 5570 letters");
        println!("21 - Nahum - 2252 letters");
        println!("22 - Habakkuk - 2598 letters");
        println!("23 - Zephaniah - 2996 letters");
        println!("24 - Haggai - 2336 letters");
        println!("25 - Zechariah - 12432 letters");
        println!("26 - Malachi - 3450 letters");
        println!("27 - Psalms - 78833 letters");
        println!("28 - Proverbs - 26507 letters");
        println!("29 - Job - 31862 letters");
        println!("30 - Song of Songs - 5151 letters");
        println!("31 - Ruth - 4947 letters");
        println!("32 - Lamentations - 5980 letters");
        println!("33 - Ecclesiastes - 10969 letters");
        println!("34 - Esther - 12112 letters");
        println!("35 - Daniel - 24291 letters");
        println!("36 - Ezra - 15764 letters");
        println!("37 - Nehemiah - 22513 letters");
        println!("38 - I Chronicles - 44558 letters");
        println!("39 - II Chronicles - 54920 letters");
        println!("\n");  // PRINT SPACE 
        println!("40 - Pentateuch (Torah) - 304850 letters");
        println!("41 - Prophets (Nevi'im) - 553785 letters");
        println!("42 - Writings (K'tuvim) - 338407 letters");
        println!("43 - Hebrew Bible (Tanach) - 1197042 letters");
        println!("\n");  // PRINT SPACE
        println!("44 - Samuel (I Samuel and II Samuel as one book) - 93532 letters");
        println!("45 - Kings (I Kings and II Kings as one book) - 98460 letters");
        println!("46 - Ezra-Nehemiah (Ezra and Nehemiah as one book) - 38277 letters");
        println!("47 - Chronicles (I Chronicles and II Chronicles as one book) - 99478 letters");
        println!("0 - QUIT / EXIT");
    
        },
        
        // PYTHON: CALL MODULE.FUNCTION() #1C - GET USER INPUT 1C- CHOOSE TEXT TO SEARCH - MAM CODEX
        3 => { NameOfCodex.push_str("Codex: 3 - MAM");
    
        // DISPLAY OPTIONS TO USER
        println!("\n");  // PRINT SPACE
        println!("Please select text to search in {}:", NameOfCodex);
        println!("\n");  // PRINT SPACE
        println!("1 - Genesis - 78063 letters");
        println!("2 - Exodus - 63527 letters");
        println!("3 - Leviticus - 44790 letters");
        println!("4 - Numbers - 63529 letters");
        println!("5 - Deuteronomy - 54892 letters");
        println!("6 - Joshua - 39730 letters");
        println!("7 - Judges - 38952 letters");
        println!("8 - I Samuel - 51357 letters");
        println!("9 - II Samuel - 42179 letters");
        println!("10 - I Kings - 50625 letters");
        println!("11 - II Kings - 47822 letters");
        println!("12 - Isaiah - 66874 letters");
        println!("13 - Jeremiah - 84899 letters");
        println!("14 - Ezekiel - 74510 letters");
        println!("15 - Hosea - 9389 letters");
        println!("16 - Joel - 3872 letters");
        println!("17 - Amos - 8034 letters");
        println!("18 - Obadiah - 1119 letters");
        println!("19 - Jonah - 2700 letters");
        println!("20 - Micah - 5571 letters");
        println!("21 - Nahum - 2255 letters");
        println!("22 - Habakkuk - 2596 letters");
        println!("23 - Zephaniah - 2995 letters");
        println!("24 - Haggai - 2336 letters");
        println!("25 - Zechariah - 12433 letters");
        println!("26 - Malachi - 3450 letters");
        println!("27 - Psalms - 78822 letters");
        println!("28 - Proverbs - 26500 letters");
        println!("29 - Job - 31851 letters");
        println!("30 - Song of Songs - 5141 letters");
        println!("31 - Ruth - 4949 letters");
        println!("32 - Lamentations - 5974 letters");
        println!("33 - Ecclesiastes - 10968 letters");
        println!("34 - Esther - 12110 letters");
        println!("35 - Daniel - 24280 letters");
        println!("36 - Ezra - 15762 letters");
        println!("37 - Nehemiah - 22507 letters");
        println!("38 - I Chronicles - 44559 letters");
        println!("39 - II Chronicles - 54917 letters");
        println!("\n");  // PRINT SPACE 
        println!("40 - Pentateuch (Torah) - 304801 letters");
        println!("41 - Prophets (Nevi'im) - 553698 letters");
        println!("42 - Writings (K'tuvim) - 338340 letters");
        println!("43 - Hebrew Bible (Tanach) - 1196839 letters");
        println!("\n");  // PRINT SPACE
        println!("44 - Samuel (I Samuel and II Samuel as one book) - 93536 letters");
        println!("45 - Kings (I Kings and II Kings as one book) - 98447 letters");
        println!("46 - Ezra-Nehemiah (Ezra and Nehemiah as one book) - 38269 letters");
        println!("47 - Chronicles (I Chronicles and II Chronicles as one book) - 99476 letters");
        println!("0 - QUIT / EXIT");
    
        },

        // CASE: CODEX: _ - UNKNOWN CODEX
        _ => { NameOfCodex.push_str("Codex: _ - Unknown Codex") }

    } // END MATCH CASE// END MATCH CASE

    // TEXT CHOSEN = USER INPUT (TEXT STRING)
    // println!("\n")  // PRINT SPACE
    // TextString = input("Please select text to search:  ")
    let mut TextString = String::new();

    // GET USER INPUT
    io::stdin()
        .read_line(&mut TextString)
        .expect("Failed to read input");
    
    // CONVERT TEXT STRING TO INTEGER
    let NumberOfTextChosen: u32 = TextString.trim().parse()
        .expect("Please enter a valid number");
    
    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("You inputted text number: {}", NumberOfTextChosen);

    // TEST PRINT OUTPUT
    // println!("\n");  // PRINT SPACE
    // println!("Selected codex: {}", NameOfCodex);

    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("WITHIN FUNCTION: END FUNCTION #1");

    // RETURN VARIABLES
    return (NameOfCodex, NumberOfTextChosen)

} 
    


