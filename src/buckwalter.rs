use std::fmt;

pub fn convert_en_ar(input: String) -> Vec<String> {
    let chars: Vec<char> = input.chars().collect();
    let mut converted = Vec::<String>::new();
    for char in chars.iter() {
        match match_char(char) {
            Ok(matched_char) => converted.push(matched_char.match_char().to_string()),
            Err(_) => (),
        };
    }
    converted
}

enum ArabicLetter {
    // The English variant names are borrowed from the Unicode Arabic definition. See: https://www.unicode.org/charts/PDF/U0600.pdf
    Space,       // Same as in English - a simple space
    Alef,        // ا
    Beh,         // ب
    Teh,         // ت
    Theh,        // ث
    Jeem,        // ج
    Hah,         // ح
    Khah,        // خ
    Dal,         // د
    Thal,        // ذ
    Reh,         // ر
    Zain,        // ز
    Seen,        // س
    Sheen,       // ش
    Sad,         // ص
    Dad,         // ض
    Tah,         // ط
    Zah,         // ظ
    Ain,         // ع
    Ghain,       // غ
    Feh,         // ف
    Qaf,         // ق
    Kaf,         // ك
    Lam,         // ل
    Meem,        // م
    Noon,        // ن
    Heh,         // ه
    Waw,         // و
    Yeh,         // ي
    AlefMaksura, // ى
    Hamza,       // ء
}

impl fmt::Display for ArabicLetter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // In my editor (NVIM with Kitty), the Arabic messes up the display at the end of each line
        // (reversing the last few characters).
        // However, the match works as intended.
        // I wrapped the Arabic letter using a regex as that was the only thing that seemed to work.
        match self {
            ArabicLetter::Space => write!(f, " "),
            ArabicLetter::Alef => write!(f, "ا"),
            ArabicLetter::Beh => write!(f, "ب"),
            ArabicLetter::Teh => write!(f, "ت"),
            ArabicLetter::Theh => write!(f, "ث"),
            ArabicLetter::Jeem => write!(f, "ج"),
            ArabicLetter::Hah => write!(f, "ح"),
            ArabicLetter::Khah => write!(f, "خ"),
            ArabicLetter::Dal => write!(f, "د"),
            ArabicLetter::Thal => write!(f, "ذ"),
            ArabicLetter::Reh => write!(f, "ر"),
            ArabicLetter::Zain => write!(f, "ز"),
            ArabicLetter::Seen => write!(f, "س"),
            ArabicLetter::Sheen => write!(f, "ش"),
            ArabicLetter::Sad => write!(f, "ص"),
            ArabicLetter::Dad => write!(f, "ض"),
            ArabicLetter::Tah => write!(f, "ط"),
            ArabicLetter::Zah => write!(f, "ظ"),
            ArabicLetter::Ain => write!(f, "ع"),
            ArabicLetter::Ghain => write!(f, "غ"),
            ArabicLetter::Feh => write!(f, "ف"),
            ArabicLetter::Qaf => write!(f, "ق"),
            ArabicLetter::Kaf => write!(f, "ك"),
            ArabicLetter::Lam => write!(f, "ل"),
            ArabicLetter::Meem => write!(f, "م"),
            ArabicLetter::Noon => write!(f, "ن"),
            ArabicLetter::Heh => write!(f, "ه"),
            ArabicLetter::Waw => write!(f, "و"),
            ArabicLetter::Yeh => write!(f, "ي"),
            ArabicLetter::AlefMaksura => write!(f, "ى"),
            ArabicLetter::Hamza => write!(f, "ء"),
        }
    }
}

enum Buckwalter {
    Letter(ArabicLetter),
}

impl Buckwalter {
    pub fn match_char(self) -> ArabicLetter {
        match self {
            Buckwalter::Letter(ArabicLetter::Space) => ArabicLetter::Space,
            Buckwalter::Letter(ArabicLetter::Alef) => ArabicLetter::Alef,
            Buckwalter::Letter(ArabicLetter::Beh) => ArabicLetter::Beh,
            Buckwalter::Letter(ArabicLetter::Teh) => ArabicLetter::Teh,
            Buckwalter::Letter(ArabicLetter::Theh) => ArabicLetter::Theh,
            Buckwalter::Letter(ArabicLetter::Jeem) => ArabicLetter::Jeem,
            Buckwalter::Letter(ArabicLetter::Hah) => ArabicLetter::Hah,
            Buckwalter::Letter(ArabicLetter::Khah) => ArabicLetter::Khah,
            Buckwalter::Letter(ArabicLetter::Dal) => ArabicLetter::Dal,
            Buckwalter::Letter(ArabicLetter::Thal) => ArabicLetter::Thal,
            Buckwalter::Letter(ArabicLetter::Reh) => ArabicLetter::Reh,
            Buckwalter::Letter(ArabicLetter::Zain) => ArabicLetter::Zain,
            Buckwalter::Letter(ArabicLetter::Seen) => ArabicLetter::Seen,
            Buckwalter::Letter(ArabicLetter::Sheen) => ArabicLetter::Sheen,
            Buckwalter::Letter(ArabicLetter::Sad) => ArabicLetter::Sad,
            Buckwalter::Letter(ArabicLetter::Dad) => ArabicLetter::Dad,
            Buckwalter::Letter(ArabicLetter::Tah) => ArabicLetter::Tah,
            Buckwalter::Letter(ArabicLetter::Zah) => ArabicLetter::Zah,
            Buckwalter::Letter(ArabicLetter::Ain) => ArabicLetter::Ain,
            Buckwalter::Letter(ArabicLetter::Ghain) => ArabicLetter::Ghain,
            Buckwalter::Letter(ArabicLetter::Feh) => ArabicLetter::Feh,
            Buckwalter::Letter(ArabicLetter::Qaf) => ArabicLetter::Qaf,
            Buckwalter::Letter(ArabicLetter::Kaf) => ArabicLetter::Kaf,
            Buckwalter::Letter(ArabicLetter::Lam) => ArabicLetter::Lam,
            Buckwalter::Letter(ArabicLetter::Meem) => ArabicLetter::Meem,
            Buckwalter::Letter(ArabicLetter::Noon) => ArabicLetter::Noon,
            Buckwalter::Letter(ArabicLetter::Heh) => ArabicLetter::Heh,
            Buckwalter::Letter(ArabicLetter::Waw) => ArabicLetter::Waw,
            Buckwalter::Letter(ArabicLetter::Yeh) => ArabicLetter::Yeh,
            Buckwalter::Letter(ArabicLetter::AlefMaksura) => ArabicLetter::AlefMaksura,
            Buckwalter::Letter(ArabicLetter::Hamza) => ArabicLetter::Hamza,
        }
    }
}

fn match_char(&char: &char) -> Result<Buckwalter, MatchError> {
    match char {
        ' ' => Ok(Buckwalter::Letter(ArabicLetter::Space)),
        'A' => Ok(Buckwalter::Letter(ArabicLetter::Alef)),
        'b' => Ok(Buckwalter::Letter(ArabicLetter::Beh)),
        't' => Ok(Buckwalter::Letter(ArabicLetter::Teh)),
        'V' => Ok(Buckwalter::Letter(ArabicLetter::Theh)),
        'j' => Ok(Buckwalter::Letter(ArabicLetter::Jeem)),
        'H' => Ok(Buckwalter::Letter(ArabicLetter::Hah)),
        'x' => Ok(Buckwalter::Letter(ArabicLetter::Khah)),
        'd' => Ok(Buckwalter::Letter(ArabicLetter::Dal)),
        '*' => Ok(Buckwalter::Letter(ArabicLetter::Thal)),
        'r' => Ok(Buckwalter::Letter(ArabicLetter::Reh)),
        'z' => Ok(Buckwalter::Letter(ArabicLetter::Zain)),
        's' => Ok(Buckwalter::Letter(ArabicLetter::Seen)),
        '$' => Ok(Buckwalter::Letter(ArabicLetter::Sheen)),
        'S' => Ok(Buckwalter::Letter(ArabicLetter::Sad)),
        'D' => Ok(Buckwalter::Letter(ArabicLetter::Dad)),
        'T' => Ok(Buckwalter::Letter(ArabicLetter::Tah)),
        'Z' => Ok(Buckwalter::Letter(ArabicLetter::Zah)),
        'E' => Ok(Buckwalter::Letter(ArabicLetter::Ain)),
        'g' => Ok(Buckwalter::Letter(ArabicLetter::Ghain)),
        'f' => Ok(Buckwalter::Letter(ArabicLetter::Feh)),
        'q' => Ok(Buckwalter::Letter(ArabicLetter::Qaf)),
        'k' => Ok(Buckwalter::Letter(ArabicLetter::Kaf)),
        'l' => Ok(Buckwalter::Letter(ArabicLetter::Lam)),
        'm' => Ok(Buckwalter::Letter(ArabicLetter::Meem)),
        'n' => Ok(Buckwalter::Letter(ArabicLetter::Noon)),
        'h' => Ok(Buckwalter::Letter(ArabicLetter::Heh)),
        'w' => Ok(Buckwalter::Letter(ArabicLetter::Waw)),
        'y' => Ok(Buckwalter::Letter(ArabicLetter::Yeh)),
        'Y' => Ok(Buckwalter::Letter(ArabicLetter::AlefMaksura)),
        '\'' => Ok(Buckwalter::Letter(ArabicLetter::Hamza)),
        _ => Err(MatchError { err: char }),
    }
}

#[derive(Debug, Clone)]
struct MatchError {
    err: char,
}

impl fmt::Display for MatchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Undefined character {}. Unable to match.", self.err)
    }
}
