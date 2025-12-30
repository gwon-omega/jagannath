//! # Anvaya - JSON Serialization (अन्वय)
//!
//! JSON parsing and serialization.
//!
//! > **"अन्वय संबन्धः पदानाम्"**
//! > *"Anvaya is the connection of words"*
//!
//! ## Etymology
//! अन्वय (anvaya) = sequence, connection, syntax

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::collections::BTreeMap;
#[cfg(feature = "alloc")]
use alloc::format;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

// ============================================================================
// JSON VALUE TYPE
// ============================================================================

/// JSON value (JSON मूल्य)
#[cfg(feature = "alloc")]
#[derive(Debug, Clone, PartialEq)]
pub enum JsonMulya {
    /// Null (शून्य)
    Shunya,
    /// Boolean (बूलीयन)
    Satya(bool),
    /// Number (संख्या)
    Sankhya(f64),
    /// String (सूत्र)
    Sutra(String),
    /// Array (सरणी)
    Sarani(Vec<JsonMulya>),
    /// Object (वस्तु)
    Vastu(BTreeMap<String, JsonMulya>),
}

#[cfg(feature = "alloc")]
impl JsonMulya {
    /// Create null
    pub fn shunya() -> Self {
        JsonMulya::Shunya
    }

    /// Create boolean
    pub fn satya(value: bool) -> Self {
        JsonMulya::Satya(value)
    }

    /// Create number from integer
    pub fn sankhya<T: Into<f64>>(value: T) -> Self {
        JsonMulya::Sankhya(value.into())
    }

    /// Create string
    pub fn sutra<S: Into<String>>(value: S) -> Self {
        JsonMulya::Sutra(value.into())
    }

    /// Create empty array
    pub fn sarani() -> Self {
        JsonMulya::Sarani(Vec::new())
    }

    /// Create array from values
    pub fn sarani_se(values: Vec<JsonMulya>) -> Self {
        JsonMulya::Sarani(values)
    }

    /// Create empty object
    pub fn vastu() -> Self {
        JsonMulya::Vastu(BTreeMap::new())
    }

    /// Create object from pairs
    pub fn vastu_se(pairs: Vec<(String, JsonMulya)>) -> Self {
        JsonMulya::Vastu(pairs.into_iter().collect())
    }

    // ========================================================================
    // TYPE CHECKS
    // ========================================================================

    /// Check if null
    pub fn shunya_hai(&self) -> bool {
        matches!(self, JsonMulya::Shunya)
    }

    /// Check if boolean
    pub fn satya_hai(&self) -> bool {
        matches!(self, JsonMulya::Satya(_))
    }

    /// Check if number
    pub fn sankhya_hai(&self) -> bool {
        matches!(self, JsonMulya::Sankhya(_))
    }

    /// Check if string
    pub fn sutra_hai(&self) -> bool {
        matches!(self, JsonMulya::Sutra(_))
    }

    /// Check if array
    pub fn sarani_hai(&self) -> bool {
        matches!(self, JsonMulya::Sarani(_))
    }

    /// Check if object
    pub fn vastu_hai(&self) -> bool {
        matches!(self, JsonMulya::Vastu(_))
    }

    // ========================================================================
    // VALUE ACCESS
    // ========================================================================

    /// Get as boolean
    pub fn satya_prapti(&self) -> Option<bool> {
        match self {
            JsonMulya::Satya(v) => Some(*v),
            _ => None,
        }
    }

    /// Get as number
    pub fn sankhya_prapti(&self) -> Option<f64> {
        match self {
            JsonMulya::Sankhya(v) => Some(*v),
            _ => None,
        }
    }

    /// Get as integer
    pub fn purnanka_prapti(&self) -> Option<i64> {
        match self {
            JsonMulya::Sankhya(v) => Some(*v as i64),
            _ => None,
        }
    }

    /// Get as string
    pub fn sutra_prapti(&self) -> Option<&str> {
        match self {
            JsonMulya::Sutra(v) => Some(v),
            _ => None,
        }
    }

    /// Get as array
    pub fn sarani_prapti(&self) -> Option<&Vec<JsonMulya>> {
        match self {
            JsonMulya::Sarani(v) => Some(v),
            _ => None,
        }
    }

    /// Get as mutable array
    pub fn sarani_prapti_mut(&mut self) -> Option<&mut Vec<JsonMulya>> {
        match self {
            JsonMulya::Sarani(v) => Some(v),
            _ => None,
        }
    }

    /// Get as object
    pub fn vastu_prapti(&self) -> Option<&BTreeMap<String, JsonMulya>> {
        match self {
            JsonMulya::Vastu(v) => Some(v),
            _ => None,
        }
    }

    /// Get as mutable object
    pub fn vastu_prapti_mut(&mut self) -> Option<&mut BTreeMap<String, JsonMulya>> {
        match self {
            JsonMulya::Vastu(v) => Some(v),
            _ => None,
        }
    }

    // ========================================================================
    // OBJECT OPERATIONS
    // ========================================================================

    /// Get field from object
    pub fn prapta(&self, key: &str) -> Option<&JsonMulya> {
        match self {
            JsonMulya::Vastu(map) => map.get(key),
            _ => None,
        }
    }

    /// Get field mutably
    pub fn prapta_mut(&mut self, key: &str) -> Option<&mut JsonMulya> {
        match self {
            JsonMulya::Vastu(map) => map.get_mut(key),
            _ => None,
        }
    }

    /// Set field in object
    pub fn sthapita(&mut self, key: String, value: JsonMulya) -> Option<JsonMulya> {
        match self {
            JsonMulya::Vastu(map) => map.insert(key, value),
            _ => None,
        }
    }

    /// Remove field from object
    pub fn hatana(&mut self, key: &str) -> Option<JsonMulya> {
        match self {
            JsonMulya::Vastu(map) => map.remove(key),
            _ => None,
        }
    }

    // ========================================================================
    // ARRAY OPERATIONS
    // ========================================================================

    /// Get array element
    pub fn tarkana(&self, index: usize) -> Option<&JsonMulya> {
        match self {
            JsonMulya::Sarani(arr) => arr.get(index),
            _ => None,
        }
    }

    /// Push to array
    pub fn yojana(&mut self, value: JsonMulya) {
        if let JsonMulya::Sarani(arr) = self {
            arr.push(value);
        }
    }

    /// Array length
    pub fn lambai(&self) -> usize {
        match self {
            JsonMulya::Sarani(arr) => arr.len(),
            JsonMulya::Vastu(map) => map.len(),
            JsonMulya::Sutra(s) => s.len(),
            _ => 0,
        }
    }
}

// ============================================================================
// JSON PARSER
// ============================================================================

/// JSON parse error (विश्लेषण दोष)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VishleshanDosha {
    /// Unexpected end of input
    AkasmikaAnta,
    /// Unexpected character
    AkasmikaAkshara(char),
    /// Invalid number
    AmanyaSankhya,
    /// Invalid string escape
    AmanyaPalayana,
    /// Invalid unicode
    AmanyaUnicode,
    /// Trailing content
    AnuvaartinaSaamagrI,
    /// Expected specific character
    ApekshitaAkshara(char),
}

#[cfg(feature = "alloc")]
struct JsonPathaka<'a> {
    input: &'a str,
    pos: usize,
}

#[cfg(feature = "alloc")]
impl<'a> JsonPathaka<'a> {
    fn nava(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    fn peek(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.pos += ch.len_utf8();
        Some(ch)
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_ascii_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn expect(&mut self, expected: char) -> Result<(), VishleshanDosha> {
        match self.advance() {
            Some(ch) if ch == expected => Ok(()),
            Some(_) => Err(VishleshanDosha::ApekshitaAkshara(expected)),
            None => Err(VishleshanDosha::AkasmikaAnta),
        }
    }

    fn parse_value(&mut self) -> Result<JsonMulya, VishleshanDosha> {
        self.skip_whitespace();

        match self.peek() {
            None => Err(VishleshanDosha::AkasmikaAnta),
            Some('n') => self.parse_null(),
            Some('t') => self.parse_true(),
            Some('f') => self.parse_false(),
            Some('"') => self.parse_string(),
            Some('[') => self.parse_array(),
            Some('{') => self.parse_object(),
            Some(ch) if ch == '-' || ch.is_ascii_digit() => self.parse_number(),
            Some(ch) => Err(VishleshanDosha::AkasmikaAkshara(ch)),
        }
    }

    fn parse_null(&mut self) -> Result<JsonMulya, VishleshanDosha> {
        self.expect('n')?;
        self.expect('u')?;
        self.expect('l')?;
        self.expect('l')?;
        Ok(JsonMulya::Shunya)
    }

    fn parse_true(&mut self) -> Result<JsonMulya, VishleshanDosha> {
        self.expect('t')?;
        self.expect('r')?;
        self.expect('u')?;
        self.expect('e')?;
        Ok(JsonMulya::Satya(true))
    }

    fn parse_false(&mut self) -> Result<JsonMulya, VishleshanDosha> {
        self.expect('f')?;
        self.expect('a')?;
        self.expect('l')?;
        self.expect('s')?;
        self.expect('e')?;
        Ok(JsonMulya::Satya(false))
    }

    fn parse_number(&mut self) -> Result<JsonMulya, VishleshanDosha> {
        let start = self.pos;

        // Optional minus
        if self.peek() == Some('-') {
            self.advance();
        }

        // Integer part
        match self.peek() {
            Some('0') => {
                self.advance();
            }
            Some(ch) if ch.is_ascii_digit() && ch != '0' => {
                while let Some(ch) = self.peek() {
                    if ch.is_ascii_digit() {
                        self.advance();
                    } else {
                        break;
                    }
                }
            }
            _ => return Err(VishleshanDosha::AmanyaSankhya),
        }

        // Fractional part
        if self.peek() == Some('.') {
            self.advance();
            let mut has_digit = false;
            while let Some(ch) = self.peek() {
                if ch.is_ascii_digit() {
                    self.advance();
                    has_digit = true;
                } else {
                    break;
                }
            }
            if !has_digit {
                return Err(VishleshanDosha::AmanyaSankhya);
            }
        }

        // Exponent part
        if let Some('e' | 'E') = self.peek() {
            self.advance();
            if let Some('+' | '-') = self.peek() {
                self.advance();
            }
            let mut has_digit = false;
            while let Some(ch) = self.peek() {
                if ch.is_ascii_digit() {
                    self.advance();
                    has_digit = true;
                } else {
                    break;
                }
            }
            if !has_digit {
                return Err(VishleshanDosha::AmanyaSankhya);
            }
        }

        let num_str = &self.input[start..self.pos];
        let num: f64 = num_str
            .parse()
            .map_err(|_| VishleshanDosha::AmanyaSankhya)?;
        Ok(JsonMulya::Sankhya(num))
    }

    fn parse_string(&mut self) -> Result<JsonMulya, VishleshanDosha> {
        self.expect('"')?;
        let mut result = String::new();

        loop {
            match self.advance() {
                None => return Err(VishleshanDosha::AkasmikaAnta),
                Some('"') => break,
                Some('\\') => {
                    match self.advance() {
                        None => return Err(VishleshanDosha::AkasmikaAnta),
                        Some('"') => result.push('"'),
                        Some('\\') => result.push('\\'),
                        Some('/') => result.push('/'),
                        Some('b') => result.push('\x08'),
                        Some('f') => result.push('\x0C'),
                        Some('n') => result.push('\n'),
                        Some('r') => result.push('\r'),
                        Some('t') => result.push('\t'),
                        Some('u') => {
                            let code = self.parse_unicode_escape()?;
                            if let Some(ch) = char::from_u32(code) {
                                result.push(ch);
                            } else {
                                // Handle surrogate pairs
                                if (0xD800..=0xDBFF).contains(&code) {
                                    // High surrogate, expect low surrogate
                                    if self.advance() != Some('\\') || self.advance() != Some('u') {
                                        return Err(VishleshanDosha::AmanyaUnicode);
                                    }
                                    let low = self.parse_unicode_escape()?;
                                    if !(0xDC00..=0xDFFF).contains(&low) {
                                        return Err(VishleshanDosha::AmanyaUnicode);
                                    }
                                    let full = 0x10000 + ((code - 0xD800) << 10) + (low - 0xDC00);
                                    if let Some(ch) = char::from_u32(full) {
                                        result.push(ch);
                                    } else {
                                        return Err(VishleshanDosha::AmanyaUnicode);
                                    }
                                } else {
                                    return Err(VishleshanDosha::AmanyaUnicode);
                                }
                            }
                        }
                        Some(_) => return Err(VishleshanDosha::AmanyaPalayana),
                    }
                }
                Some(ch) if ch < '\x20' => return Err(VishleshanDosha::AkasmikaAkshara(ch)),
                Some(ch) => result.push(ch),
            }
        }

        Ok(JsonMulya::Sutra(result))
    }

    fn parse_unicode_escape(&mut self) -> Result<u32, VishleshanDosha> {
        let mut code = 0u32;
        for _ in 0..4 {
            let digit = match self.advance() {
                Some(ch) if ch.is_ascii_hexdigit() => ch.to_digit(16).unwrap(),
                _ => return Err(VishleshanDosha::AmanyaUnicode),
            };
            code = (code << 4) | digit;
        }
        Ok(code)
    }

    fn parse_array(&mut self) -> Result<JsonMulya, VishleshanDosha> {
        self.expect('[')?;
        self.skip_whitespace();

        let mut result = Vec::new();

        if self.peek() == Some(']') {
            self.advance();
            return Ok(JsonMulya::Sarani(result));
        }

        loop {
            let value = self.parse_value()?;
            result.push(value);

            self.skip_whitespace();
            match self.peek() {
                Some(',') => {
                    self.advance();
                }
                Some(']') => {
                    self.advance();
                    break;
                }
                _ => return Err(VishleshanDosha::ApekshitaAkshara(']')),
            }
        }

        Ok(JsonMulya::Sarani(result))
    }

    fn parse_object(&mut self) -> Result<JsonMulya, VishleshanDosha> {
        self.expect('{')?;
        self.skip_whitespace();

        let mut result = BTreeMap::new();

        if self.peek() == Some('}') {
            self.advance();
            return Ok(JsonMulya::Vastu(result));
        }

        loop {
            self.skip_whitespace();

            // Parse key
            let key = match self.parse_string()? {
                JsonMulya::Sutra(s) => s,
                _ => unreachable!(),
            };

            self.skip_whitespace();
            self.expect(':')?;

            // Parse value
            let value = self.parse_value()?;
            result.insert(key, value);

            self.skip_whitespace();
            match self.peek() {
                Some(',') => {
                    self.advance();
                }
                Some('}') => {
                    self.advance();
                    break;
                }
                _ => return Err(VishleshanDosha::ApekshitaAkshara('}')),
            }
        }

        Ok(JsonMulya::Vastu(result))
    }
}

/// Parse JSON string (JSON विश्लेषण)
#[cfg(feature = "alloc")]
pub fn vishleshan(input: &str) -> Result<JsonMulya, VishleshanDosha> {
    let mut parser = JsonPathaka::nava(input);
    let value = parser.parse_value()?;
    parser.skip_whitespace();

    if parser.pos != parser.input.len() {
        return Err(VishleshanDosha::AnuvaartinaSaamagrI);
    }

    Ok(value)
}

// ============================================================================
// JSON SERIALIZER
// ============================================================================

/// Stringify JSON value (JSON सूत्रीकरण)
#[cfg(feature = "alloc")]
pub fn sutrikarana(value: &JsonMulya) -> String {
    let mut output = String::new();
    write_value(value, &mut output, false, 0);
    output
}

/// Stringify JSON with pretty printing
#[cfg(feature = "alloc")]
pub fn sutrikarana_sundara(value: &JsonMulya) -> String {
    let mut output = String::new();
    write_value(value, &mut output, true, 0);
    output
}

#[cfg(feature = "alloc")]
fn write_value(value: &JsonMulya, output: &mut String, pretty: bool, indent: usize) {
    match value {
        JsonMulya::Shunya => output.push_str("null"),
        JsonMulya::Satya(true) => output.push_str("true"),
        JsonMulya::Satya(false) => output.push_str("false"),
        JsonMulya::Sankhya(n) => {
            if n.is_finite() {
                if n.fract() == 0.0 && n.abs() < 1e15 {
                    output.push_str(&format!("{}", *n as i64));
                } else {
                    output.push_str(&format!("{}", n));
                }
            } else {
                output.push_str("null"); // JSON doesn't support infinity/NaN
            }
        }
        JsonMulya::Sutra(s) => write_string(s, output),
        JsonMulya::Sarani(arr) => {
            output.push('[');
            if !arr.is_empty() {
                if pretty {
                    output.push('\n');
                }
                for (i, item) in arr.iter().enumerate() {
                    if i > 0 {
                        output.push(',');
                        if pretty {
                            output.push('\n');
                        }
                    }
                    if pretty {
                        for _ in 0..indent + 2 {
                            output.push(' ');
                        }
                    }
                    write_value(item, output, pretty, indent + 2);
                }
                if pretty {
                    output.push('\n');
                    for _ in 0..indent {
                        output.push(' ');
                    }
                }
            }
            output.push(']');
        }
        JsonMulya::Vastu(map) => {
            output.push('{');
            if !map.is_empty() {
                if pretty {
                    output.push('\n');
                }
                for (i, (key, val)) in map.iter().enumerate() {
                    if i > 0 {
                        output.push(',');
                        if pretty {
                            output.push('\n');
                        }
                    }
                    if pretty {
                        for _ in 0..indent + 2 {
                            output.push(' ');
                        }
                    }
                    write_string(key, output);
                    output.push(':');
                    if pretty {
                        output.push(' ');
                    }
                    write_value(val, output, pretty, indent + 2);
                }
                if pretty {
                    output.push('\n');
                    for _ in 0..indent {
                        output.push(' ');
                    }
                }
            }
            output.push('}');
        }
    }
}

#[cfg(feature = "alloc")]
fn write_string(s: &str, output: &mut String) {
    output.push('"');
    for ch in s.chars() {
        match ch {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            '\x08' => output.push_str("\\b"),
            '\x0C' => output.push_str("\\f"),
            ch if ch < '\x20' => {
                output.push_str(&format!("\\u{:04x}", ch as u32));
            }
            ch => output.push(ch),
        }
    }
    output.push('"');
}

// ============================================================================
// BUILDER PATTERN
// ============================================================================

/// JSON object builder (वस्तु निर्माता)
#[cfg(feature = "alloc")]
pub struct VastuNirmata {
    map: BTreeMap<String, JsonMulya>,
}

#[cfg(feature = "alloc")]
impl VastuNirmata {
    /// Create new builder
    pub fn nava() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    /// Add string field
    pub fn sutra<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.map.insert(key.into(), JsonMulya::Sutra(value.into()));
        self
    }

    /// Add number field
    pub fn sankhya<K: Into<String>>(mut self, key: K, value: f64) -> Self {
        self.map.insert(key.into(), JsonMulya::Sankhya(value));
        self
    }

    /// Add integer field
    pub fn purnanka<K: Into<String>>(mut self, key: K, value: i64) -> Self {
        self.map
            .insert(key.into(), JsonMulya::Sankhya(value as f64));
        self
    }

    /// Add boolean field
    pub fn satya<K: Into<String>>(mut self, key: K, value: bool) -> Self {
        self.map.insert(key.into(), JsonMulya::Satya(value));
        self
    }

    /// Add null field
    pub fn shunya<K: Into<String>>(mut self, key: K) -> Self {
        self.map.insert(key.into(), JsonMulya::Shunya);
        self
    }

    /// Add value field
    pub fn mulya<K: Into<String>>(mut self, key: K, value: JsonMulya) -> Self {
        self.map.insert(key.into(), value);
        self
    }

    /// Add nested object
    pub fn vastu<K: Into<String>>(mut self, key: K, builder: VastuNirmata) -> Self {
        self.map.insert(key.into(), builder.nirmana());
        self
    }

    /// Add array
    pub fn sarani<K: Into<String>>(mut self, key: K, builder: SaraniNirmata) -> Self {
        self.map.insert(key.into(), builder.nirmana());
        self
    }

    /// Build the object
    pub fn nirmana(self) -> JsonMulya {
        JsonMulya::Vastu(self.map)
    }
}

/// JSON array builder (सरणी निर्माता)
#[cfg(feature = "alloc")]
pub struct SaraniNirmata {
    arr: Vec<JsonMulya>,
}

#[cfg(feature = "alloc")]
impl SaraniNirmata {
    /// Create new builder
    pub fn nava() -> Self {
        Self { arr: Vec::new() }
    }

    /// Add string
    pub fn sutra<V: Into<String>>(mut self, value: V) -> Self {
        self.arr.push(JsonMulya::Sutra(value.into()));
        self
    }

    /// Add number
    pub fn sankhya(mut self, value: f64) -> Self {
        self.arr.push(JsonMulya::Sankhya(value));
        self
    }

    /// Add integer
    pub fn purnanka(mut self, value: i64) -> Self {
        self.arr.push(JsonMulya::Sankhya(value as f64));
        self
    }

    /// Add boolean
    pub fn satya(mut self, value: bool) -> Self {
        self.arr.push(JsonMulya::Satya(value));
        self
    }

    /// Add null
    pub fn shunya(mut self) -> Self {
        self.arr.push(JsonMulya::Shunya);
        self
    }

    /// Add value
    pub fn mulya(mut self, value: JsonMulya) -> Self {
        self.arr.push(value);
        self
    }

    /// Add nested object
    pub fn vastu(mut self, builder: VastuNirmata) -> Self {
        self.arr.push(builder.nirmana());
        self
    }

    /// Add nested array
    pub fn sarani(mut self, builder: SaraniNirmata) -> Self {
        self.arr.push(builder.nirmana());
        self
    }

    /// Build the array
    pub fn nirmana(self) -> JsonMulya {
        JsonMulya::Sarani(self.arr)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "alloc")]
    fn test_parse_null() {
        assert_eq!(vishleshan("null").unwrap(), JsonMulya::Shunya);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_parse_bool() {
        assert_eq!(vishleshan("true").unwrap(), JsonMulya::Satya(true));
        assert_eq!(vishleshan("false").unwrap(), JsonMulya::Satya(false));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_parse_numbers() {
        assert_eq!(vishleshan("0").unwrap(), JsonMulya::Sankhya(0.0));
        assert_eq!(vishleshan("123").unwrap(), JsonMulya::Sankhya(123.0));
        assert_eq!(vishleshan("-456").unwrap(), JsonMulya::Sankhya(-456.0));
        assert_eq!(vishleshan("3.14").unwrap(), JsonMulya::Sankhya(3.14));
        assert_eq!(vishleshan("1e10").unwrap(), JsonMulya::Sankhya(1e10));
        assert_eq!(vishleshan("2.5e-3").unwrap(), JsonMulya::Sankhya(2.5e-3));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_parse_strings() {
        assert_eq!(
            vishleshan(r#""hello""#).unwrap(),
            JsonMulya::Sutra("hello".into())
        );
        assert_eq!(
            vishleshan(r#""hello\nworld""#).unwrap(),
            JsonMulya::Sutra("hello\nworld".into())
        );
        assert_eq!(
            vishleshan(r#""tab\there""#).unwrap(),
            JsonMulya::Sutra("tab\there".into())
        );
        assert_eq!(
            vishleshan(r#""\u0048\u0065\u006c\u006c\u006f""#).unwrap(),
            JsonMulya::Sutra("Hello".into())
        );
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_parse_arrays() {
        assert_eq!(vishleshan("[]").unwrap(), JsonMulya::Sarani(vec![]));
        assert_eq!(
            vishleshan("[1, 2, 3]").unwrap(),
            JsonMulya::Sarani(vec![
                JsonMulya::Sankhya(1.0),
                JsonMulya::Sankhya(2.0),
                JsonMulya::Sankhya(3.0),
            ])
        );
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_parse_objects() {
        let json = r#"{"name": "John", "age": 30}"#;
        let parsed = vishleshan(json).unwrap();

        assert!(parsed.vastu_hai());
        assert_eq!(
            parsed.prapta("name").unwrap().sutra_prapti().unwrap(),
            "John"
        );
        assert_eq!(
            parsed.prapta("age").unwrap().sankhya_prapti().unwrap(),
            30.0
        );
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_stringify_roundtrip() {
        let json = r#"{"array":[1,2,3],"bool":true,"null":null,"number":42,"string":"hello"}"#;
        let parsed = vishleshan(json).unwrap();
        let stringified = sutrikarana(&parsed);
        let reparsed = vishleshan(&stringified).unwrap();

        assert_eq!(parsed, reparsed);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_builder() {
        let obj = VastuNirmata::nava()
            .sutra("name", "Test")
            .purnanka("count", 42)
            .satya("active", true)
            .sarani(
                "items",
                SaraniNirmata::nava().purnanka(1).purnanka(2).purnanka(3),
            )
            .nirmana();

        let json = sutrikarana(&obj);
        let parsed = vishleshan(&json).unwrap();

        assert_eq!(
            parsed.prapta("name").unwrap().sutra_prapti().unwrap(),
            "Test"
        );
        assert_eq!(
            parsed.prapta("count").unwrap().purnanka_prapti().unwrap(),
            42
        );
        assert_eq!(
            parsed.prapta("active").unwrap().satya_prapti().unwrap(),
            true
        );
        assert_eq!(parsed.prapta("items").unwrap().lambai(), 3);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_pretty_print() {
        let obj = VastuNirmata::nava()
            .sutra("name", "Test")
            .purnanka("value", 42)
            .nirmana();

        let pretty = sutrikarana_sundara(&obj);
        assert!(pretty.contains('\n'));
        assert!(pretty.contains("  ")); // indentation
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_string_escaping() {
        let obj = JsonMulya::Sutra("line1\nline2\ttab\"quote\\backslash".into());
        let json = sutrikarana(&obj);
        assert!(json.contains("\\n"));
        assert!(json.contains("\\t"));
        assert!(json.contains("\\\""));
        assert!(json.contains("\\\\"));

        let parsed = vishleshan(&json).unwrap();
        assert_eq!(parsed, obj);
    }
}
