#[allow(dead_code)]

// IMPORT MODULES

/// LetterObject struct - represents a single letter with all its attributes
/// Equivalent to the Python cls_LetterObject class
/// /// #[allow(clippy::too_many_arguments)]
#[derive(Debug, Clone)]
pub struct LetterObject {
    pub Letter: Option<char>,
    pub LetterGematriaNumberValue: Option<u32>,
    pub LetterPositionIndex: Option<u32>,
    pub LetterCoordinatesD5K: Option<(u32, u32, u32, u32, u32)>,  // (BOOK#, CHAPTER#, VERSE#, LETTER#INVERSE, LETTER#INTEXT)
    pub LetterCoordinatesDL: Option<(u32, u32, u32, u32)>,          // (BOOK#, CHAPTER#, VERSE#, LETTER#INVERSE)
    pub LetterPositionInWord: Option<u32>,
    pub WordNumber: Option<u32>,
    pub Word: Option<String>,
    pub WordNumberInVerse: Option<u32>,
    pub WordCoordinatesDWTK: Option<(u32, u32, u32, u32, u32)>,
    pub VerseCoordinatesDS: Option<(u32, u32, u32)>,
    pub Verse: Option<String>,
    // ELS-RELATED FIELDS
    // pub IsMatchInELS: Option<bool>,
    // pub NumberOfMatches: Option<u32>,
    // pub ListOfMatches: Option<Vec<u32>>,
    // pub DictOfMatches: Option<IndexMap<String, u32>>,
}

impl LetterObject {
    /// Constructor for LetterObject
    /// All parameters are optional, defaulting to None
    /// #[allow(clippy::too_many_arguments)]
    
    /// Simplified constructor with only essential parameters
    pub fn fn_ConstructLO(

        Letter: char,
        LetterPositionIndex: u32,
        LetterCoordinatesD5K: (u32, u32, u32, u32, u32),
        LetterCoordinatesDL: (u32, u32, u32, u32),
        VerseCoordinatesDS: (u32, u32, u32),

    ) -> Self {

        LetterObject {

            Letter: Some(Letter),
            LetterGematriaNumberValue: None,
            LetterPositionIndex: Some(LetterPositionIndex),
            LetterCoordinatesD5K: Some(LetterCoordinatesD5K),
            LetterCoordinatesDL: Some(LetterCoordinatesDL),
            LetterPositionInWord: None,
            WordNumber: None,
            Word: None,
            WordNumberInVerse: None,
            WordCoordinatesDWTK: None,
            VerseCoordinatesDS: Some(VerseCoordinatesDS),
            Verse: None,
        //    IsMatchInELS: None,
        //    NumberOfMatches: None,
        //    ListOfMatches: None,
        //    DictOfMatches: None,

        }
    }
}

/*
impl fmt::Display for LetterObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "LetterObject {{ Letter: {:?}, Position: {:?} }}",
            self.Letter, self.LetterPositionIndex
        )
    }
}
*/