/// A module for calculating the frequency and percentage statistics of Hebrew letters in a selected text.
///
/// MODULE.FUNCTION() #6 - CALCULATE LETTER PERCENTAGES; RETURNS -> LIST OF TUPLES OF LETTER STATISTICS
///
/// This module provides functionality to analyze a Hebrew text sequence and compute statistics for each Hebrew letter,
/// including the count of occurrences, total text length, and percentage representation. It processes a text string to calculate
/// letter frequencies and percentages for each Hebrew letter, including both regular and final forms (e.g., כ/ך, מ/ם).
///
/// This function iterates through a provided text string, counts the occurrences of each Hebrew letter, and calculates
/// the percentage of each letter relative to the total number of characters. It handles both regular and final forms of letters
/// (e.g., כ and ך are counted separately and combined as כ/ך). The results are returned as a vector of tuples containing
/// the letter, its count, the total text length, and its decimal and percentage representations.
///
/// # Arguments
/// * `s` - A `&str` representing the input text sequence of Hebrew letters to analyze.
///
/// # Returns
/// A `Vec<(String, u32, u32, f64, f64)>` where each tuple contains:
/// - The Hebrew letter (or combined form, e.g., "כ/ך") as a `String`.
/// - The number of occurrences of the letter in the text as a `u32`.
/// - The total number of characters in the text as a `u32`.
/// - The percentage as a decimal `f64`.
/// - The percentage as a percentage % == (decimal * 100) as an `f64`.
///
/// Please see the CSV files that are produced for example output:
///
/// # Examples
/// ```
/// // CALL MODULE.FUNCTION() #6 - CALCULATE LETTER PERCENTAGES; RETURNS -> LIST OF TUPLES OF LETTER STATISTICS
/// let ListOfTuplesOfLetterStatistics = mod_6_CalculateLetterPercentages::fn_CalculatePercentages(&S);
/// ```
///
/// # Panics
/// This function does not panic under normal circumstances. It handles empty input strings by returning
/// percentages of 0.0 for all letters and assumes valid Unicode Hebrew characters in the input.
///
/// MODULE.FUNCTION() #6 - CALCULATE LETTER PERCENTAGES; RETURNS -> LIST OF TUPLES OF LETTER STATISTICS
/// 
// BEGIN FUNCTION() #6 - CALCULATE LETTER PERCENTAGES; RETURNS -> LIST OF TUPLES OF LETTER STATISTICS
pub fn fn_CalculatePercentages(s: &str) -> Vec<(String, u32, u32, f64, f64)> {

    // ## TEST PRINT OUTPUT
    println!("\n"); // PRINT SPACE
    println!("WITHIN FUNCTION:  BEGIN FUNCTION #6 - CALCULATE LETTER PERCENTAGES");

    // COUNT OCCURRENCES OF EACH LETTER
    let Letter1 = ("א".to_string(), s.chars().filter(|&c| c == 'א').count() as u32);
    let Letter2 = ("ב".to_string(), s.chars().filter(|&c| c == 'ב').count() as u32);
    let Letter3 = ("ג".to_string(), s.chars().filter(|&c| c == 'ג').count() as u32);
    let Letter4 = ("ד".to_string(), s.chars().filter(|&c| c == 'ד').count() as u32);
    let Letter5 = ("ה".to_string(), s.chars().filter(|&c| c == 'ה').count() as u32);
    let Letter6 = ("ו".to_string(), s.chars().filter(|&c| c == 'ו').count() as u32);
    let Letter7 = ("ז".to_string(), s.chars().filter(|&c| c == 'ז').count() as u32);
    let Letter8 = ("ח".to_string(), s.chars().filter(|&c| c == 'ח').count() as u32);
    let Letter9 = ("ט".to_string(), s.chars().filter(|&c| c == 'ט').count() as u32);
    let Letter10 = ("י".to_string(), s.chars().filter(|&c| c == 'י').count() as u32);
    let Letter11A = ("כ".to_string(), s.chars().filter(|&c| c == 'כ').count() as u32);
    let Letter11B = ("ך".to_string(), s.chars().filter(|&c| c == 'ך').count() as u32);
    let Letter11 = ("כ/ך".to_string(), Letter11A.1 + Letter11B.1);

    let Letter12 = ("ל".to_string(), s.chars().filter(|&c| c == 'ל').count() as u32);
    let Letter13A = ("מ".to_string(), s.chars().filter(|&c| c == 'מ').count() as u32);
    let Letter13B = ("ם".to_string(), s.chars().filter(|&c| c == 'ם').count() as u32);
    let Letter13 = ("מ/ם".to_string(), Letter13A.1 + Letter13B.1);

    let Letter14A = ("נ".to_string(), s.chars().filter(|&c| c == 'נ').count() as u32);
    let Letter14B = ("ן".to_string(), s.chars().filter(|&c| c == 'ן').count() as u32);
    let Letter14 = ("נ/ן".to_string(), Letter14A.1 + Letter14B.1);

    let Letter15 = ("ס".to_string(), s.chars().filter(|&c| c == 'ס').count() as u32);
    let Letter16 = ("ע".to_string(), s.chars().filter(|&c| c == 'ע').count() as u32);
    let Letter17A = ("פ".to_string(), s.chars().filter(|&c| c == 'פ').count() as u32);
    let Letter17B = ("ף".to_string(), s.chars().filter(|&c| c == 'ף').count() as u32);
    let Letter17 = ("פ/ף".to_string(), Letter17A.1 + Letter17B.1);

    let Letter18A = ("צ".to_string(), s.chars().filter(|&c| c == 'צ').count() as u32);
    let Letter18B = ("ץ".to_string(), s.chars().filter(|&c| c == 'ץ').count() as u32);
    let Letter18 = ("צ/ץ".to_string(), Letter18A.1 + Letter18B.1);

    let Letter19 = ("ק".to_string(), s.chars().filter(|&c| c == 'ק').count() as u32);
    let Letter20 = ("ר".to_string(), s.chars().filter(|&c| c == 'ר').count() as u32);
    let Letter21 = ("ש".to_string(), s.chars().filter(|&c| c == 'ש').count() as u32);
    let Letter22 = ("ת".to_string(), s.chars().filter(|&c| c == 'ת').count() as u32);

    // COLLECT ALL LETTER TUPLES
    let ListOfTuplesOfLetterInstances = vec![
        Letter1, Letter2, Letter3, Letter4, Letter5, Letter6, Letter7, Letter8, Letter9, Letter10,
        Letter11A, Letter11B, Letter11, Letter12, Letter13A, Letter13B, Letter13, Letter14A, Letter14B,
        Letter14, Letter15, Letter16, Letter17A, Letter17B, Letter17, Letter18A, Letter18B, Letter18,
        Letter19, Letter20, Letter21, Letter22,
    ];

    // DECLARE VARIABLES
    // CREATE NEW VECTOR / LIST OF TUPLES
    let mut ListOfTuplesOfLetterStatistics = Vec::new();

    // GET LENGTH OF TEXT (NUMBER OF UNICODE CHARACTERS) 
    let LengthOfText = s.chars().count() as u32;

    // ## BEGIN FOR LOOP
    for EachLetterTuple in ListOfTuplesOfLetterInstances {

        // ## TEST PRINT OUTPUT
        // println!("EachLetterTuple: {:?}", EachLetterTuple);

        //
        let NumberOfLetterInstances = EachLetterTuple.1;
        
        //
        let PercentageOfLetterInTextAsDecimal = if LengthOfText > 0 {
            NumberOfLetterInstances as f64 / LengthOfText as f64
        } else {
            0.0
        };

        //
        let PercentageOfLetterInTextAsPercentage = PercentageOfLetterInTextAsDecimal * 100.0;

        //
        let TempTuple = (
            EachLetterTuple.0,
            NumberOfLetterInstances,
            LengthOfText,
            PercentageOfLetterInTextAsDecimal,
            PercentageOfLetterInTextAsPercentage,
        );

        //
        ListOfTuplesOfLetterStatistics.push(TempTuple);
    }
    // ## END FOR LOOP

    // ## TEST PRINT OUTPUT
    println!("\n"); // PRINT SPACE
    println!("WITHIN FUNCTION:  END FUNCTION #6 - CALCULATE LETTER PERCENTAGES");

    // ## RETURN VARIABLE(S)
    ListOfTuplesOfLetterStatistics

}
// END FUNCTION() #6 - CALCULATE LETTER PERCENTAGES; RETURNS -> LIST OF TUPLES OF LETTER STATISTICS