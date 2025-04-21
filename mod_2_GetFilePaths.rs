// IMPORT MODULES
/// A module for retrieving file paths to JSON data files for Hebrew Bible codices.
/// 
/// MODULE.FUNCTION() #2 - GET FILE PATHS FOR SOURCE TEXTS; RETURNS -> (STRING, STRING)
/// 
/// This module provides functionality to determine the file paths for JSON data files
/// containing Hebrew Bible texts based on the selected codex. It retrieves file paths
/// for JSON data files based on the selected Hebrew Bible codex.
/// 
/// This function takes a codex number, determines the appropriate JSON file paths for
/// the Dictionary of Verses (With No Spaces) (D) and Dictionary of Verses (With Spaces) (DS) files corresponding to the selected codex, and
/// returns a tuple containing these two file paths.
/// 
/// # Arguments
/// * `NumberOfCodex` - A `u32` representing the selected codex (1: Koren, 2: Leningrad, 3: MAM).
/// 
/// # Returns
/// A tuple `(String, String)` containing:
/// - The file path for the Dictionary of Verses (With No Spaces) file (D) as a `String`: `D` = Dictionary of Verses (With No Spaces)
/// - The file path for the Dictionary of Verses (With Spaces) file (DS) as a `String`: `DS` = Dictionary of Verses (With Spaces)
/// 
/// For Codex 1 - Koren:
/// - `1`: Koren Codex - Dictionary of Verses (With No Spaces) file (D): "../src/texts/JSONData_D_Koren.json"
/// - `1`: Koren Codex - Dictionary of Verses (With Spaces) file (DS): "../src/texts/JSONData_DS_Koren.json"
/// 
/// For Codex 2 - Leningrad:
/// - `2`: Leningrad Codex - Dictionary of Verses (With No Spaces) file (D): "../src/texts/JSONData_D_Leningrad.json"
/// - `2`: Leningrad Codex - Dictionary of Verses (With Spaces) file (DS): "../src/texts/JSONData_DS_Leningrad.json"
/// 
/// For Codex 3 - MAM:
/// - `3`: MAM Codex - Dictionary of Verses (With No Spaces) file (D): "../src/texts/JSONData_D_MAM.json"
/// - `3`: MAM Codex - Dictionary of Verses (With Spaces) file (DS): "../src/texts/JSONData_DS_MAM.json"
/// 
/// The codices used here are the same codices used in the TorahBibleCodes Python software:
/// <https://GitHub.com/TorahBibleCodes/TorahBibleCodes>
/// 
/// However, for sake of development efficiency and uniformity of codex data structures, it was deemed suitable to export
/// the organized codex data as JSON files from the Python version of the program, and then open the JSON files in Rust:
/// this instead of parsing three separate codices with three separate raw formats that will take much more development time in Rust,
/// i.e. when this has already been done and solved with Python, and thus this step of preparing the raw data into uniform Hebrew for all three codices can be skipped
/// in the transposition of the software into Rust. 
/// 
/// # Examples
/// ```
/// // SIMULATING FUNCTION CALL THAT SELECTS CODEX 1 - KOREN:
/// // CALL MODULE.FUNCTION() #2 - GET FILE PATHS FOR SOURCE TEXTS; RETURNS -> (STRING, STRING)
/// let (FilePath_D, FilePath_DS) = mod_2_GetFilePaths::fn_GetFilePaths(1);
/// // FilePath_D would be "../src/texts/JSONData_D_Koren.json"
/// // FilePath_DS would be "../src/texts/JSONData_DS_Koren.json"
/// ```
/// 
/// # Panics
/// This function does not panic under normal circumstances. Invalid codex numbers are
/// handled by returning empty strings and printing an error message to stderr.
/// 
/// MODULE.FUNCTION() #2 - GET FILE PATHS FOR SOURCE TEXTS; RETURNS -> (STRING, STRING)
/// 
// BEGIN FUNCTION() #2 - GET FILE PATHS FOR SOURCE TEXTS; RETURNS -> (STRING, STRING)
pub fn fn_GetFilePaths(NumberOfCodex: u32) -> (String, String) {

    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("WITHIN FUNCTION:  BEGIN FUNCTION #2 - GET FILE PATHS FOR SOURCE TEXTS");
    
    // Print status (optional)
    println!("Opening JSON file...");
    println!("Current dir: {:?}", std::env::current_dir());

    // DECLARE VARIABLES
    let FilePath_D: String;
    let FilePath_DS: String;
    // "../src/texts/JSONData_D_Koren.json"
    // "../src/texts/JSONData_DS_Koren.json"
    // "../src/texts/JSONData_D_Leningrad.json"
    // "../src/texts/JSONData_DS_Leningrad.json"
    // "../src/texts/JSONData_D_MAM.json"
    // "../src/texts/JSONData_DS_MAM.json"
    
    match NumberOfCodex {

        // KOREN
        1 => { FilePath_D = String::from("../src/texts/JSONData_D_Koren.json"); 
               FilePath_DS = String::from("../src/texts/JSONData_DS_Koren.json");
        },
        
        // LENINGRAD
        2 => { FilePath_D = String::from("../src/texts/JSONData_D_Leningrad.json");
               FilePath_DS = String::from("../src/texts/JSONData_DS_Leningrad.json");
        },

        // MAM
        3 => { FilePath_D = String::from("../src/texts/JSONData_D_MAM.json");
               FilePath_DS = String::from("../src/texts/JSONData_DS_MAM.json");
        },

        // INVALID CASE
        _ => {
            eprintln!("❌ ERROR: Invalid NumberOfCodex value: {}", NumberOfCodex);
            FilePath_D = String::from(""); // return blank or handle error as needed
            FilePath_DS = String::from(""); // return blank or handle error as needed
        }
    };

    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("WITHIN FUNCTION:  END FUNCTION #2 - GET FILE PATHS FOR SOURCE TEXTS");

    // RETURN FILE PATH
    return (FilePath_D, FilePath_DS);

}
// END FUNCTION() #2 - GET FILE PATHS FOR SOURCE TEXTS; RETURNS -> (STRING, STRING)