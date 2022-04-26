use std::fmt;
use std::borrow::Cow;

use rand::{self, Rng};

/// Table to retrieve base62 values from.
const BASE62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

/// A _probably_ unique paste ID.
pub struct PasteId<'a>(Cow<'a, str>);

impl<'a> PasteId<'a> {
    /// Generate a _probably_ unique ID with `size` characters. For readability,
    /// the characters used are from the sets [0-9], [A-Z], [a-z]. The
    /// probability of a collision depends on the value of `size` and the number
    /// of IDs generated thus far.
    pub fn new(size: usize) -> PasteId<'static> {
        let mut id = String::with_capacity(size);
        let mut rng = rand::thread_rng();
        for _ in 0..size {
            id.push(BASE62[rng.gen::<usize>() % 62] as char);
        }

        PasteId(Cow::Owned(id))
    }
}

impl<'a> fmt::Display for PasteId<'a> {

  fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self)
  }
}