// IMPORT MODULES
use indexmap::IndexMap;

// ENUM TO HANDLE DICT OR LIST INPUT
#[allow(dead_code)] // THIS IS NECESSARY BECAUSE FIRST TIME THERE IS NO LIST PASSED, AND RUST RAISES WARNINGS IN COMPILATION
pub enum DictOrList<T, U> {
    Dict(IndexMap<(u32, u32, u32, u32, u32), T>),
    List(Vec<U>),
}

// FUNCTION() #13 - LIST OF INDEXES (CUSTOM) CREATE
pub fn fn_ListOfIndexesCustomCreate<T, U>(DictOrList: DictOrList<T, U>) -> Vec<u32> {
    
    /*
    MODULE.FUNCTION() #13 - CREATE LIST OF CUSTOM INDEXES FOR EACH LETTER IN SELECTED TEXT NON-0-INDEXED / 1-INDEXED  RETURNS ListOfIndexesCustom
    */
    
    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("WITHIN FUNCTION:  BEGIN FUNCTION #13 - LIST OF INDEXES (CUSTOM) CREATE");
          
    // DECLARE VARIABLES
    // EMPTY LIST TO HOLD CUSTOM INDEX NUMBERS TO BE NON-0-INDEXED / 1-INDEXED
    let mut ListOfIndexesCustom: Vec<u32> = Vec::new();
    
    // MATCH ON DictOrList ENUM
    match DictOrList {
        // IF PARAMETER IS DICTIONARY, THEN DO THIS...
        DictOrList::Dict(dict) => {
            
            // FOR EACH 5-DIGIT TUPLE KEY IN D5...
            for each in dict.keys() { // EACH == KEY OF D5
            
                // TEST PRINT OUTPUT
                // print("index = ", each[-1])
                // print("each D5 key =", each)
                // println!("index = {}", each.4);
                // println!("each D5 key = {:?}", each);
            
                // DECLARE INDEX VALUE
                let IndexCustom = each.4;
            
                // ADD CUSTOM INDEX TO LIST OF CUSTOM INDEXES
                ListOfIndexesCustom.push(IndexCustom);
            }
        }
        
        // IF PARAMETER IS LIST, THEN DO THIS
        DictOrList::List(list) => {
            
            // CREATE COUNTER FOR THE LIST INDEXES
            let mut IndexCustom = 1;
            
            // FOR EACH ELEMENT IN LIST...
            for each in list {
                
                // ADD CUSTOM INDEX TO LIST OF CUSTOM INDEXES
                ListOfIndexesCustom.push(IndexCustom);
                
                // INCREMENT INDEX COUNTER
                IndexCustom += 1;
            }
        }
    }
    
    // TEST PRINT OUTPUT        
    // print(ListOfIndexesCustom)
    // println!("{:?}", ListOfIndexesCustom);
    
    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("WITHIN FUNCTION:  END FUNCTION #13 - LIST OF INDEXES (CUSTOM) CREATE");
    
    // RETURN VARIABLES
    return ListOfIndexesCustom;
}

// END FUNCTION() #13 - LIST OF INDEXES (CUSTOM) CREATE