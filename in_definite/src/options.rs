pub struct Options {
    /// If true, then a 4 digit number like '1800' is treated like 'eighteen hundred', so will use 'an'.
    /// 
    /// Normally, such a number is treated like 'one thousand eight hundred', so would use 'a'.
    pub are_numbers_colloquial: bool,
}

impl Options {
    pub fn default() -> Options {
        Options {are_numbers_colloquial: false}
    }

    pub fn with_colloquial() -> Options {
        Options {are_numbers_colloquial: true}
    }
}
