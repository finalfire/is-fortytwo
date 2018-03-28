/// `Is` is the core component of the library.
/// It encapsulates an `i64` number.
/// Note that it could be different from 42, although that's would be out of the library scope.
pub struct Is {
    number: i64
}

impl Is {
    /// Create a new `Is`. To be executed by `Is::new(n)`.
    pub fn new(n: i64) -> Is {
        Is {number: n}
    }

    /// Check whether the number encapsulated by `Is` is fourtytwo or neither.
    pub fn fourtytwo(self) -> bool {
        self.number == 42
    }

    /// Adds `n` to the number encapsulated by `Is`.
    /// Returns a new `Is`.
    pub fn plus(self, n: i64) -> Is {
        Is {number: self.number + n}
    }

    /// Substracts `n` to the number encapsulated by `Is`.
    /// Returns a new `Is`.
    pub fn minus(self, n: i64) -> Is {
        Is {number: self.number - n}
    }

    /// Multiplies `n` to the number encapsulated by `Is`.
    /// Returns a new `Is`.
    pub fn times(self, n: i64) -> Is {
        Is {number: self.number * n}
    }
}