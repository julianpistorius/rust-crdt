use std::collections::BTreeSet;

use rustc_serialize::{Encodable, Decodable};

/// A `GSet` is a grow-only set.
#[derive(Debug, Clone, PartialEq, Eq, Hash, RustcEncodable, RustcDecodable)]
pub struct GSet<A: Ord + Encodable + Decodable> {
    value: BTreeSet<A>,
}

impl<A: Ord + Encodable + Decodable> GSet<A> {
    /// Instantiates an empty `GSet`.
    pub fn new() -> GSet<A> {
        GSet {
            value: BTreeSet::new()
        }
    }

    /// Merges another `GSet` into this one.
    ///
    /// # Examples
    ///
    /// ```
    /// use crdts::GSet;
    /// let (mut a, mut b) = (GSet::new(), GSet::new());
    /// a.insert(1);
    /// b.insert(2);
    /// a.merge(b);
    /// assert!(a.contains(&1));
    /// assert!(a.contains(&2));
    /// ```
    pub fn merge(&mut self, other: GSet<A>) {
        for e in other.value.into_iter() {
            self.insert(e);
        }
    }

    /// Inserts an element into this `GSet`.
    ///
    /// # Examples
    ///
    /// ```
    /// use crdts::GSet;
    /// let mut a = GSet::new();
    /// a.insert(1);
    /// assert!(a.contains(&1));
    /// ```
    pub fn insert(&mut self, element: A) {
        self.value.insert(element);
    }

    /// Returns `true` if the `GSet` contains the element.
    ///
    /// # Examples
    ///
    /// ```
    /// use crdts::GSet;
    /// let mut a = GSet::new();
    /// a.insert(1);
    /// assert!(a.contains(&1));
    /// ```
    pub fn contains(&self, element: &A) -> bool {
        self.value.contains(element)
    }
}
