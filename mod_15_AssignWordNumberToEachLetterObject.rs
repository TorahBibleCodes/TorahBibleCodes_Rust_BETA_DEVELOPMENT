// IMPORT MODULES
use indexmap::IndexMap;
use crate::mod_cls_99_LetterObject::LetterObject;

// FUNCTION() #15 - ASSIGN WORD NUMBER TO EACH LETTER OBJECT
pub fn fn_AssignWordNumberToEachLetterObject<'a>(
    DLO: &'a mut IndexMap<u32, LetterObject>,
    DW: &IndexMap<u32, (String, Option<Vec<u32>>, (u32, Vec<u32>, u32))>,
    DWTK: &IndexMap<u32, (u32, u32, u32, u32, u32)>
) -> &'a mut IndexMap<u32, LetterObject> {
    
    /*
    MODULE.FUNCTION() #15 - ASSIGN WORD NUMBER TO EACH LETTER OBJECT
    */
    
    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("WITHIN FUNCTION:  BEGIN FUNCTION #15 - ASSIGN WORD NUMBER TO EACH LETTER OBJECT");
    
    // DECLARE VARIABLES FOR FOR LOOP BELOW WITH 1-INDEX DW OBJECT
    // DW[1] == ('בראשית', [1, 2, 3, 4, 5, 6], (1, [2, 200, 1, 300, 10, 400], 913))
    
    for (key, EachWordTuple) in DW.iter() { // 1-BASED KEY, VALUE: EachWordTuple == DW[1] == ('בראשית', [1, 2, 3, 4, 5, 6], (1, [2, 200, 1, 300, 10, 400], 913))
        
        if let Some(indices) = &EachWordTuple.1 { // 2ND ITEM IN 0-BASED TUPLE: [1, 2, 3, 4, 5, 6]
            
            for &EachIndexPosition in indices { // 2ND ITEM IN 0-BASED TUPLE: [1, 2, 3, 4, 5, 6]
                
                if EachIndexPosition == DLO.get(&EachIndexPosition).unwrap().LetterPositionIndex.unwrap() {
                    
                    if let Some(letter_obj) = DLO.get_mut(&EachIndexPosition) {
                        letter_obj.WordNumber = Some(*key);
                        
                        // TEST
                        letter_obj.WordCoordinatesDWTK = DWTK.get(key).cloned();
                    }
                }
            }
        }
    }
    
    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("WITHIN FUNCTION:  END FUNCTION #15 - ASSIGN WORD NUMBER TO EACH LETTER OBJECT");
    
    // RETURN VARIABLES
    return DLO ;
}

// END FUNCTION() #15 - ASSIGN WORD NUMBER TO EACH LETTER OBJECT