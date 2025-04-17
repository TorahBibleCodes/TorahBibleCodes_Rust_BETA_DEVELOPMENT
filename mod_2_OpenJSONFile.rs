// IMPORT MODULES
use indexmap::IndexMap;
use std::fs;
// use serde::Deserialize;


use serde_json;

// FUNCTION() #2A - OPEN JSON FILE
pub fn fn_OpenJSONFile(NumberOfCodex: u32) -> (String, String) {

    /*
    // MODULE.FUNCTION() #2 - OPEN JSON FILE
    */

    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("WITHIN FUNCTION:  BEGIN FUNCTION #2A - OPEN JSON FILE");
    
    // Print status (optional)
    println!("Opening JSON file...");
    println!("Current dir: {:?}", std::env::current_dir());

    // DECLARE VARIABLES
    let FilePath_D: String;
    let FilePath_DS: String;
    // "src/texts/JSONData_D_Koren.json"
    // "src/texts/JSONData_DS_Koren.json"
    // "src/texts/JSONData_D_Leningrad.json"
    // "src/texts/JSONData_DS_Leningrad.json"
    // "src/texts/JSONData_D_MAM.json"
    // "src/texts/JSONData_DS_MAM.json"
    
    match NumberOfCodex {

        // KOREN
        1 => { FilePath_D = String::from("src/texts/JSONData_D_Koren.json"); 
               FilePath_DS = String::from("src/texts/JSONData_DS_Koren.json");
        },
        
        // LENINGRAD
        2 => { FilePath_D = String::from("src/texts/JSONData_D_Leningrad.json");
               FilePath_DS = String::from("src/texts/JSONData_DS_Leningrad.json");
        },

        // MAM
        3 => { FilePath_D = String::from("src/texts/JSONData_D_MAM.json");
               FilePath_DS = String::from("src/texts/JSONData_DS_MAM.json");
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
    println!("WITHIN FUNCTION:  END FUNCTION #2A - OPEN JSON FILE");

    // RETURN FILE PATH
    return (FilePath_D, FilePath_DS);

}
// END FUNCTION() #2A - OPEN JSON FILE

// BEGIN FUNCTION() #2B - LOAD AND PARSE JSON
pub fn fn_LoadAndParseJSON(FilePath: String) -> IndexMap<(u32, u32, u32), String> {

    // PARSE JSON DATA INTO INDEXMAP

    // Step 1: Try reading the file
    let file_content = match fs::read_to_string(&FilePath) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("❌ Failed to read file '{}': {}", FilePath, e);
            return IndexMap::new();
        }
    };

    // Step 2: Try parsing the JSON
    let raw: IndexMap<String, String> = match serde_json::from_str(&file_content) {
        Ok(map) => map,
        Err(e) => {
            eprintln!("❌ Failed to parse JSON from '{}': {}", FilePath, e);
            return IndexMap::new();
        }
    };

    // Step 3: Convert keys into tuple form and populate final HashMap
    let mut verse_map: IndexMap<(u32, u32, u32), String> = IndexMap::new();

    for (key, val) in raw {
        let parts: Vec<&str> = key.split(',').collect();

        if parts.len() == 3 {
            if let (Ok(b), Ok(c), Ok(v)) = (
                parts[0].trim().parse::<u32>(),
                parts[1].trim().parse::<u32>(),
                parts[2].trim().parse::<u32>(),
            ) {
                verse_map.insert((b, c, v), val);
            } else {
                eprintln!("❌ Could not parse integers from key: {}", key);
            }
        } else {
            eprintln!("❌ Key format invalid (expected 3 parts): {}", key);
        }
    }

    // Return the final map
    verse_map

}
// END FUNCTION() #2B - LOAD AND PARSE JSON
