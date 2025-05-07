// IMPORT MODULES
use std::fs::File;
use std::io::{BufReader, BufRead};

// FUNCTION() #24 - READ INPUT FROM FILE: CSV ELS SEARCH TERMS
pub fn fn_ReadInputFromFile(FileNameForCSVImport: String) -> (Vec<String>, Vec<String>, u32) {
    
    /*
    MODULE.FUNCTION() #24 -
    */
    
    // DEFINE VARIABLES
    let mut ListOfSearchTermsWithSpaces: Vec<String> = Vec::new(); // LIST OF ROWS
    let mut ListOfSearchTerms: Vec<String> = Vec::new(); // LIST OF ROWS
    
    // OPEN (IF EXISTS) CSV FILE; READ INPUT FROM CSV FILE
    let file = File::open(&FileNameForCSVImport)
        .expect("Failed to open CSV file");
    
    // CREATE BUFFERED READER
    let reader = BufReader::new(file);
    
    // BEGIN FOR LOOP - ITERATE THROUGH LINES
    for line_result in reader.lines() {
        let line = line_result.expect("Failed to read line");
        
        // Split line by delimiter ';'
        let fields: Vec<&str> = line.split(';').collect();
        
        // EXTRACT first field
        let ELSSearchTermTextWithSpaces = fields.get(0)
            .expect("CSV row missing first column")
            .to_string();
        
        // APPEND
        ListOfSearchTermsWithSpaces.push(ELSSearchTermTextWithSpaces.clone());
        
        // DEAL WITH SPACES IN ELS SEARCH TERMS - REMOVE SPACES FROM STRING
        let ELSSearchTermText = ELSSearchTermTextWithSpaces.replace(" ", "");
        
        // APPEND
        ListOfSearchTerms.push(ELSSearchTermText);
    }
    // END FOR LOOP - ITERATE THROUGH LINES
    
    // COUNT NUMBER OF SEARCH TERMS
    let NumberOfSearchTerms = ListOfSearchTerms.len() as u32;
    
    // RETURN VARIABLES
    (ListOfSearchTerms, ListOfSearchTermsWithSpaces, NumberOfSearchTerms)
}

// END FUNCTION() #24 - READ INPUT FROM FILE: CSV ELS SEARCH TERMS