// IMPORT MODULES
use indexmap::IndexMap;
use crate::mod_cls_99_LetterObject::LetterObject;

// FUNCTION () #9AAA - ##
pub fn fn_AddGematriaNumberValuesToLetterObjects(mut DLO: IndexMap<u32, LetterObject>, N: Vec<u32>) -> IndexMap<u32, LetterObject> {

    /*
    ## MODULE.FUNCTION() #11 - ADD GEMATRIA NUMBER VALUES TO EACH LETTER OBJECT (LO) IN DICTIONARY OF LETTER OBJECTS (DLO)
    */

    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("WITHIN FUNCTION:  BEGIN FUNCTION #11 - ADD GEMATRIA NUMBER VALUES TO EACH LETTER OBJECT (LO) IN DICTIONARY OF LETTER OBJECTS (DLO)");

    // TEST - ADD GEMATRIA NUMBER VALUES TO EACH LETTER OBJECT (LO) IN DICTIONARY OF LETTER OBJECTS (DLO)

    // DECLARE VARIABLES - FOR FOR LOOP BELOW WITH 0-INDEX N OBJECT
    let mut CurrentLetterIndexPosition = 0;
    let mut TotalLetterCount = 1;

    // ADDING LETTER NUMBER VALUE PROPERTY VALUE TO EACH INSTANCE OF LETTER OBJECT
    for EachLetterObject in DLO.values_mut() {

        // GET VALUE OF NUMBER BY INDEX START:STOP ## THIS CREATES A LIST OF ONE ELEMENT ONLY
        let LetterGematriaNumberValue = &N[CurrentLetterIndexPosition..TotalLetterCount];

        // EXTRACT INTEGER FROM THE LIST OF ELEMENT ## [20] --> 20
        let LetterGematriaNumberValue = LetterGematriaNumberValue[0];

        // TEST PRINT OUTPUT
        // println!("\n");  // PRINT SPACE
        // println!("TEST PRINT OUTPUT {} {}", std::any::type_name_of_val(&LetterGematriaNumberValue), LetterGematriaNumberValue);

        // ADD VALUE OF NUMBER TO INSTANCE OF LETTER OBJECT
        EachLetterObject.LetterGematriaNumberValue = Some(LetterGematriaNumberValue);

        // INCREMENT
        CurrentLetterIndexPosition += 1;
        TotalLetterCount += 1;
    }

    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("WITHIN FUNCTION:  END FUNCTION #11 - ADD GEMATRIA NUMBER VALUES TO EACH LETTER OBJECT (LO) IN DICTIONARY OF LETTER OBJECTS (DLO)");

    // RETURN VARIABLES
    return DLO;

}

// END FUNCTION () #11 -