// IMPORT MODULES
use std::io;

// FUNCTION () #0 - GET USER INPUT; CHOOSE CODEX TO SEARCH; // RETURNS INTEGER //
pub fn fn_GetUserInput() -> u32 {

    /*
    // MODULE.FUNCTION() #0 - GET USER INPUT; CHOOSE CODEX TO SEARCH; // RETURNS INTEGER //
     */

    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("WITHIN FUNCTION:  BEGIN FUNCTION #0 - GET USER INPUT; CHOOSE CODEX TO SEARCH; // RETURNS INTEGER");
    
    // GET USER INPUT
    println!("\n");  // PRINT SPACE
    println!("Please select Hebrew Bible codex to search:");
    println!("\n");  // PRINT SPACE
    println!("1 - Koren Codex - Claremont Michigan Transliteration: [Torah: 304805]");
      
    println!("\n");  // PRINT SPACE
    println!("2 - Leningrad Codex: [Torah: 304850; Prophets: 553785; Writings: 338407; Tanach: 1197042]");

    println!("\n");  // PRINT SPACE
    println!("3 - Miqra According to the Masorah (MAM) Collection of Manuscripts: [Torah: 304801; Prophets: 553698; Writings: 338340; Tanach: 1196839]");
   
    println!("\n");  // PRINT SPACE
    println!("0 - QUIT / EXIT");

    // TEXT CHOSEN = USER INPUT (TEXT STRING)
    println!("\n"); // PRINT SPACE
    // TextString = input("Please select codex (collection of manuscripts) to search (1: Koren; 2: Leningrad; 3: MAM):  ")
    let mut TextString = String::new();

    // GET USER INPUT
    io::stdin()
        .read_line(&mut TextString)
        .expect("Failed to read input");
    
    // CONVERT TEXT STRING TO INTEGER
    let NumberOfCodexChosen: u32 = TextString.trim().parse()
        .expect("Please enter a valid number");
    
    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("You inputted the number: {}", NumberOfCodexChosen);
    
    // CONVERT TEXT STRING TO INTEGER
    // NumberOfCodexChosen = int(TextString) // PYTHON
    // let NumberOfCodexChosen: u8 = 1;  // 8-bit unsigned integer (0 to 255)
    // let NumberOfCodexChosen: u8 = TextString.parse().unwrap();
    
    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    // println!(f"You have chosen codex number # {NumberOfCodexChosen}."); // PYTHON
    println!("You have chosen codex number # {}.", NumberOfCodexChosen);

    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("WITHIN FUNCTION:  END FUNCTION #0 - GET USER INPUT; CHOOSE CODEX TO SEARCH; // RETURNS INTEGER");

    // RETURN VARIABLES TO PROGRAM
    return NumberOfCodexChosen

}

// END FUNCTION () #0 - GET USER INPUT; CHOOSE CODEX TO SEARCH

