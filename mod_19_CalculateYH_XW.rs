// FUNCTION() #19 - CALCULATE XW AND YH FOR THE 2D MATRIX CSV FILE
pub fn fn_CalculateYH_XW(FactorY: u32, FactorX: u32, ListOfFactors: &Vec<u32>, LLL: &mut Vec<char>, LengthOfTextToSearch: u32) -> (u32, u32, Vec<char>) {
    
    /*
    MODULE.FUNCTION() #19 - CALCULATE XW AND YH FOR THE 2D MATRIX CSV FILE - RETURNS YH, XW, LLL
    */
    
    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("WITHIN FUNCTION:  BEGIN FUNCTION #19 - CALCULATE XW / YH");
    
    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("FactorY = {}", FactorY); // THIS VALUE MUST BE CORRECTED IF NOT PERFECT FACTORIAL
    println!("FactorX = {}", FactorX);
    
    // TEST - FACTORS - EXAMPLE: TEXT OF BOOK OF GENESIS
    // XXX = 159 // NECESSARY FOR CALCULATING ABSOLUTE X BELOW
    // YYY = 491
    
    // IF FACTORY AND FACTORX ARE IN LIST OF PERFECT FACTORS/DIVISORS FOR LENGTH OF THE TEXT...
    let YH: u32;
    let XW: u32;
    
    if ListOfFactors.contains(&FactorY) && ListOfFactors.contains(&FactorX) {
        println!("True - The X / W / #COLUMNS you have chosen is in the list of factors of this text length of {} letters", LengthOfTextToSearch);
        YH = FactorY;
        XW = FactorX;
    } else {
        println!("False - The X / W / #COLUMNS you have chosen is NOT in the list of factors of this text length of {} letters", LengthOfTextToSearch);
        
        // THEN RECALCULATE FOR 2D MATRIX OUTPUT BUG IF NOT CORRECTED...
        
        // TEST
        // GET ABSOLUTE VALUE OF DIFFERENCE BETWEEN USER CHOICE AND FACTOR X
        // AbsX = abs(FactorX - XXX) // 50 - 159 == 109
        // println!("AbsX {}", AbsX);
        
        // TEST - EXAMPLE: TEXT OF BOOK OF GENESIS
        // AreaToSliceOff = (AbsX * FactorY) // 109 * 491 == 53519 == AREA THAT WE MUST SLICE OFF TO RECALCULATE
        // 78069 - 53519 == 24550 // == AREA LEFT AFTER WE SLICE OFF OTHER AREA
        // 78069 / 50 == 1561.38
        // 78069 % 50 == 19 // REMAINDER AFTER MODULO OPERATION
        // 50 * 1561 == 78050 // MISSING REMAINDER OF 19
        
        // GET AREA OF SYMMETRICAL 2D MATRIX FOR SELECTED TEXT
        let AreaOf2DMatrix = LengthOfTextToSearch;
        println!("AreaOf2DMatrix BEFORE USER CHOICE OF X / W / #COLUMNS {}", AreaOf2DMatrix);
        
        // GET REMAINDER FROM USER CHOICE OF FACTOR X / W / HORIZONTAL ROWS FOR 2D MATRIX CSV FILE
        let RemainderAfterUserChoice = LengthOfTextToSearch % FactorX;
        println!("RemainderAfterUserChoice {}", RemainderAfterUserChoice);
        
        // CALCULATE # OF #EMPTY SPACES NECESSARY IN LAST ROW OF 2D MATRIX IF USER CHOICE OF FACTOR X IX NOT PERFECT FACTOR
        let SpaceToFillInLastRow = FactorX - RemainderAfterUserChoice;
        println!("SpaceToFillInLastRow {}", SpaceToFillInLastRow);
        
        // ADD SPACES TO END OF TEXT STRING IN ORDER TO KEEP XW VS. YH CONSISTENTLY SYMMETRICAL
        let NewLengthOfTextToOutput = LengthOfTextToSearch + SpaceToFillInLastRow; // 78100 == (78069 + 31)
        println!("NewLengthOfTextToOutput {}", NewLengthOfTextToOutput);
        
        // CALCULATE NEW YH / COLUMNS
        YH = NewLengthOfTextToOutput / FactorX;
        println!("YH {}", YH);
        
        // CALCULATE NEW XW / ROWS
        XW = FactorX; // Already an integer
        println!("XW {}", XW);
        
        // DECLARE VARIABLES
        let mut CCC: u32 = 0; // CREATE COUNTER VARIABLE
        
        // ADD SPACES TO END OF TEXT STRING IN ORDER TO KEEP XW VS. YH CONSISTENTLY SYMMETRICAL
        while CCC < SpaceToFillInLastRow {
            
            // APPEND BLANK SPACE TO LIST OF LETTERS IN SELECTED TEXT (FOR THE FINAL ROW TO FILL WITH BLANK SPACES AFTER RECALCULATION)
            LLL.push(' ');
            
            // INCREMENT CCC COUNTER VARIABLE
            CCC += 1;
        }
        
        // TEST PRINT OUTPUT
        println!("\n");  // PRINT SPACE
        println!("Length of LLL {}", LLL.len());
    }
    
    // TEST PRINT OUTPUT
    println!("\n");  // PRINT SPACE
    println!("WITHIN FUNCTION:  END FUNCTION #19 - CALCULATE XW / YH");
    
    // RETURN VARIABLES TO PROGRAM
    return(YH, XW, LLL.clone());
}

// END FUNCTION() #19 - CALCULATE XW AND YH FOR THE 2D MATRIX CSV FILE