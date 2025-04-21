// IMPORT MODULES
use indexmap::IndexMap;
use std::fs;

/// A module for loading and parsing JSON data files containing Hebrew Bible texts.
/// 
/// MODULE.FUNCTION() #3 - LOAD AND PARSE JSON; RETURNS -> INDEXMAP<(INTEGER, INTEGER, INTEGER), STRING>
/// 
/// This module provides functionality to read JSON files and parse their contents into
/// an indexed map, mapping verse references to text strings. It loads and parses a JSON file 
/// into an indexed map (i.e. `D` and `DS`) of verse references to text strings.
/// 
/// This function reads a JSON file from the specified file path, parses its contents into
/// an `IndexMap`, and converts the keys (verse references from the text format (`"Book#,Chapter#,Verse#"`)
/// into tuples of three unsigned 32-bit integers, i.e. `(Book#, Chapter#, Verse#)`.
/// 
/// It returns an `IndexMap` mapping these 3-integer tuple keys to the corresponding verses of text strings
/// (both without spaces (`D`), and with spaces (`DS`)). It handles errors by printing messages to stderr and
/// returning an empty map for invalid files or JSON.
/// 
/// # Arguments
/// * `FilePath` - A `String` specifying the path to the JSON file to load.
/// 
/// # Returns
/// An `IndexMap<(u32, u32, u32), String>` where:
/// - The key is a tuple `(Book#, Chapter#, Verse#)` of three `u32` values representing the verse reference.
/// - The value is a `String` containing the text for that verse.
/// 
/// Example JSON input for Dictionary of Verses (No Spaces) (`D`):
/// - `"1,1,1": "בראשיתבראאלהיםאתהשמיםואתהארץ"`
/// - `"1,1,2": "והארץהיתהתהוובהווחשךעלפניתהוםורוחאלהיםמרחפתעלפניהמים"`
/// - `"1,1,3": "ויאמראלהיםיהיאורויהיאור"`
/// 
/// Example JSON input for Dictionary of Verses (With Spaces) (`DS`):
/// - `"1,1,1": "בראשית ברא אלהים את השמים ואת הארץ"`
/// - `"1,1,2": "והארץ היתה תהו ובהו וחשך על פני תהום ורוח אלהים מרחפת על פני המים"`
/// - `"1,1,3": "ויאמר אלהים יהי אור ויהי אור"`
/// 
/// Returns an empty `IndexMap` if:
/// - The file cannot be read.
/// - The JSON cannot be parsed.
/// - The keys are not in the correct format (three comma-separated integers).
/// 
/// # Examples
/// ```
/// // CALL MODULE.FUNCTION() #3 - LOAD AND PARSE JSON; RETURNS -> INDEXMAP<(INTEGER, INTEGER, INTEGER), STRING>
/// let VerseMap_D = mod_3_LoadAndParseJSON::fn_LoadAndParseJSON(FilePath);
/// ```
/// 
/// # Panics
/// This function does not panic under normal circumstances. It handles errors by printing
/// error messages to stderr and returning an empty `IndexMap` for invalid files, JSON, or
/// key formats.
/// 
/// MODULE.FUNCTION() #3 - LOAD AND PARSE JSON; RETURNS -> INDEXMAP<(INTEGER, INTEGER, INTEGER), STRING>
/// 
// BEGIN FUNCTION() #3 - LOAD AND PARSE JSON; RETURNS -> INDEXMAP<(INTEGER, INTEGER, INTEGER), STRING>
pub fn fn_LoadAndParseJSON(FilePath: String) -> IndexMap<(u32, u32, u32), String> {
      
    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("WITHIN FUNCTION:  BEGIN FUNCTION #3 - LOAD AND PARSE JSON");
    
    // PARSE JSON DATA INTO INDEXMAP

    // STEP 1: TRY READING THE FILE
    let FileContent = match fs::read_to_string(&FilePath) {
        
        Ok(content) => content,
        
        Err(e) => {

            eprintln!("❌ Failed to read file '{}': {}", FilePath, e);
            
            return IndexMap::new();
        }
    };

    // STEP 2: TRY PARSING THE JSON
    let raw: IndexMap<String, String> = match serde_json::from_str(&FileContent) {
        
        Ok(map) => map,
        
        Err(e) => {

            eprintln!("❌ Failed to parse JSON from '{}': {}", FilePath, e);
            
            return IndexMap::new();
        
        }
    };

    // STEP 3: CONVERT KEYS TO TUPLE AND POPULATE FINAL INDEX (HASH) MAP
    let mut VerseMap: IndexMap<(u32, u32, u32), String> = IndexMap::new();

    // BEGIN FOR LOOP
    for (key, val) in raw {

        //
        let parts: Vec<&str> = key.split(',').collect();

        // BEGIN IF / ELSE
        if parts.len() == 3 {

            // BEGIN IF / ELSE
            if let (Ok(b), Ok(c), Ok(v)) = (

                parts[0].trim().parse::<u32>(),
                parts[1].trim().parse::<u32>(),
                parts[2].trim().parse::<u32>(),

            ) {

                VerseMap.insert((b, c, v), val);
            
            } else {
            
                eprintln!("❌ Could not parse integers from key: {}", key);
            
            }
            // END IF / ELSE

        } else {
            
            eprintln!("❌ Key format invalid (expected 3 parts): {}", key);
        
        }
        // END IF / ELSE
    }
    // END FOR LOOP

    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("WITHIN FUNCTION:  END FUNCTION #3 - LOAD AND PARSE JSON");
    
    // RETURN FINAL MAP
    VerseMap

}
// END FUNCTION() #3 - LOAD AND PARSE JSON; RETURNS -> INDEXMAP<(INTEGER, INTEGER, INTEGER), STRING>
