//! Note: Most of the documentation is taken from
//! rusts hashmap.rs and should be considered under
//! their copyright.

use crate::vecmap::{self, Entry as VecMapEntry};
use core::hash::{BuildHasher, Hash};
use hashbrown::{
    self,
    hash_map::{self, Entry as HashBrownEntry},
};
use std::fmt;

/////// General

/// A view into a single entry in a map, which may either be vacant or occupied.
///
/// This `enum` is constructed from the [`entry`] method on [`HashMap`].
///
/// [`HashMap`]: struct.HashMap.html
/// [`entry`]: struct.HashMap.html#method.entry
pub enum Entry<'a, K, V, S>
where
    S: BuildHasher,
{
    /// An occupied entry.
    Occupied(OccupiedEntry<'a, K, V, S>),

    /// A vacant entry.
    Vacant(VacantEntry<'a, K, V, S>),
}

impl<'a, K, V, S> Entry<'a, K, V, S>
where
    S: BuildHasher,
{
    /// Ensures a value is in the entry by inserting the default if empty, and returns
    /// a mutable reference to the value in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use halfbrown::HashMap;
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    ///
    /// map.entry("poneyland").or_insert(3);
    /// assert_eq!(map["poneyland"], 3);
    ///
    /// *map.entry("poneyland").or_insert(10) *= 2;
    /// assert_eq!(map["poneyland"], 6);
    /// ```
    #[inline]
    pub fn or_insert(self, default: V) -> &'a mut V
    where
        K: Hash,
        S: BuildHasher,
    {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default),
        }
    }

    /// Ensures a value is in the entry by inserting the result of the default function if empty,
    /// and returns a mutable reference to the value in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use halfbrown::HashMap;
    ///
    /// let mut map: HashMap<&str, String> = HashMap::new();
    /// let s = "hoho".to_string();
    ///
    /// map.entry("poneyland").or_insert_with(|| s);
    ///
    /// assert_eq!(map["poneyland"], "hoho".to_string());
    /// ```
    #[inline]
    pub fn or_insert_with<F: FnOnce() -> V>(self, default: F) -> &'a mut V
    where
        K: Hash,
        S: BuildHasher,
    {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default()),
        }
    }

    /// Returns a reference to this entry's key.
    ///
    /// # Examples
    ///
    /// ```
    /// use halfbrown::HashMap;
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    /// assert_eq!(map.entry("poneyland").key(), &"poneyland");
    /// ```
    #[inline]
    pub fn key(&self) -> &K {
        match *self {
            Entry::Occupied(ref entry) => entry.key(),
            Entry::Vacant(ref entry) => entry.key(),
        }
    }

    /// Provides in-place mutable access to an occupied entry before any
    /// potential inserts into the map.
    ///
    /// # Examples
    ///
    /// ```
    /// use halfbrown::HashMap;
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    ///
    /// map.entry("poneyland")
    ///    .and_modify(|e| { *e += 1 })
    ///    .or_insert(42);
    /// assert_eq!(map["poneyland"], 42);
    ///
    /// map.entry("poneyland")
    ///    .and_modify(|e| { *e += 1 })
    ///    .or_insert(42);
    /// assert_eq!(map["poneyland"], 43);
    /// ```
    #[inline]
    pub fn and_modify<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut V),
    {
        match self {
            Entry::Occupied(mut entry) => {
                f(entry.get_mut());
                Entry::Occupied(entry)
            }
            Entry::Vacant(entry) => Entry::Vacant(entry),
        }
    }
}

impl<'a, K, V, S> From<HashBrownEntry<'a, K, V, S>> for Entry<'a, K, V, S>
where
    S: BuildHasher,
{
    fn from(f: HashBrownEntry<'a, K, V, S>) -> Entry<'a, K, V, S> {
        match f {
            HashBrownEntry::Occupied(o) => Entry::Occupied(OccupiedEntry(OccupiedEntryInt::Map(o))),
            HashBrownEntry::Vacant(o) => Entry::Vacant(VacantEntry(VacantEntryInt::Map(o))),
        }
    }
}

impl<'a, K, V, S> From<VecMapEntry<'a, K, V, S>> for Entry<'a, K, V, S>
where
    S: BuildHasher,
{
    fn from(f: VecMapEntry<'a, K, V, S>) -> Entry<'a, K, V, S> {
        match f {
            VecMapEntry::Occupied(o) => Entry::Occupied(OccupiedEntry(OccupiedEntryInt::Vec(o))),
            VecMapEntry::Vacant(o) => Entry::Vacant(VacantEntry(VacantEntryInt::Vec(o))),
        }
    }
}

impl<K: fmt::Debug, V: fmt::Debug, S> fmt::Debug for Entry<'_, K, V, S>
where
    S: BuildHasher,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Entry::Vacant(ref v) => f.debug_tuple("Entry").field(v).finish(),
            Entry::Occupied(ref o) => f.debug_tuple("Entry").field(o).finish(),
        }
    }
}

/// A view into an occupied entry in a `HashMap`.
/// It is part of the [`Entry`] enum.
///
/// [`Entryx`]: enum.Entry.html
pub struct OccupiedEntry<'a, K, V, S>(OccupiedEntryInt<'a, K, V, S>)
where
    S: BuildHasher;

enum OccupiedEntryInt<'a, K, V, S>
where
    S: BuildHasher,
{
    Map(hash_map::OccupiedEntry<'a, K, V, S>),
    Vec(vecmap::OccupiedEntry<'a, K, V, S>),
}

unsafe impl<K, V, S> Send for OccupiedEntry<'_, K, V, S>
where
    K: Send,
    V: Send,
    S: Send + BuildHasher,
{
}
unsafe impl<K, V, S> Sync for OccupiedEntry<'_, K, V, S>
where
    K: Sync,
    V: Sync,
    S: Sync + BuildHasher,
{
}

impl<K: fmt::Debug, V: fmt::Debug, S> fmt::Debug for OccupiedEntry<'_, K, V, S>
where
    S: BuildHasher,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            OccupiedEntryInt::Map(m) => write!(f, "{:?}", m),
            OccupiedEntryInt::Vec(m) => write!(f, "{:?}", m),
        }
    }
}

/// A view into a vacant entry in a `HashMap`.
/// It is part of the [`Entry`] enum.
///
/// [`Entry`]: enum.Entry.html
pub struct VacantEntry<'a, K, V, S>(VacantEntryInt<'a, K, V, S>)
where
    S: BuildHasher;

enum VacantEntryInt<'a, K, V, S>
where
    S: BuildHasher,
{
    /// a map based implementation
    Map(hashbrown::hash_map::VacantEntry<'a, K, V, S>),
    /// a vec based implementation
    Vec(vecmap::VacantEntry<'a, K, V, S>),
}

impl<K: fmt::Debug, V, S> fmt::Debug for VacantEntry<'_, K, V, S>
where
    S: BuildHasher,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            VacantEntryInt::Map(m) => write!(f, "{:?}", m),
            VacantEntryInt::Vec(m) => write!(f, "{:?}", m),
        }
    }
}

impl<'a, K, V, S> OccupiedEntry<'a, K, V, S>
where
    S: BuildHasher,
{
    /// Gets a reference to the key in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use halfbrown::HashMap;
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    /// map.entry("poneyland").or_insert(12);
    /// assert_eq!(map.entry("poneyland").key(), &"poneyland");
    /// ```
    #[inline]
    pub fn key(&self) -> &K {
        match &self.0 {
            OccupiedEntryInt::Map(m) => m.key(),
            OccupiedEntryInt::Vec(m) => m.key(),
        }
    }

    /// Take the ownership of the key and value from the map.
    ///
    /// # Examples
    ///
    /// ```
    /// use halfbrown::HashMap;
    /// use halfbrown::Entry;
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    /// map.entry("poneyland").or_insert(12);
    ///
    /// if let Entry::Occupied(o) = map.entry("poneyland") {
    ///     // We delete the entry from the map.
    ///     o.remove_entry();
    /// }
    ///
    /// assert_eq!(map.contains_key("poneyland"), false);
    /// ```
    #[inline]
    pub fn remove_entry(self) -> (K, V) {
        match self.0 {
            OccupiedEntryInt::Map(m) => m.remove_entry(),
            OccupiedEntryInt::Vec(m) => m.remove_entry(),
        }
    }

    /// Gets a reference to the value in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use halfbrown::HashMap;
    /// use halfbrown::Entry;
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    /// map.entry("poneyland").or_insert(12);
    ///
    /// if let Entry::Occupied(o) = map.entry("poneyland") {
    ///     assert_eq!(o.get(), &12);
    /// }
    /// ```
    #[inline]
    pub fn get(&self) -> &V {
        match &self.0 {
            OccupiedEntryInt::Map(m) => m.get(),
            OccupiedEntryInt::Vec(m) => m.get(),
        }
    }

    /// Gets a mutable reference to the value in the entry.
    ///
    /// If you need a reference to the `OccupiedEntry` which may outlive the
    /// destruction of the `Entry` value, see [`into_mut`].
    ///
    /// [`into_mut`]: #method.into_mut
    ///
    /// # Examples
    ///
    /// ```
    /// use halfbrown::HashMap;
    /// use halfbrown::Entry;
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    /// map.entry("poneyland").or_insert(12);
    ///
    /// assert_eq!(map["poneyland"], 12);
    /// if let Entry::Occupied(mut o) = map.entry("poneyland") {
    ///     *o.get_mut() += 10;
    ///     assert_eq!(*o.get(), 22);
    ///
    ///     // We can use the same Entry multiple times.
    ///     *o.get_mut() += 2;
    /// }
    ///
    /// assert_eq!(map["poneyland"], 24);
    /// ```
    #[inline]
    pub fn get_mut(&mut self) -> &mut V {
        match &mut self.0 {
            OccupiedEntryInt::Map(m) => m.get_mut(),
            OccupiedEntryInt::Vec(m) => m.get_mut(),
        }
    }

    /// Converts the `OccupiedEntry` into a mutable reference to the value in the entry
    /// with a lifetime bound to the map itself.
    ///
    /// If you need multiple references to the `OccupiedEntry`, see [`get_mut`].
    ///
    /// [`get_mut`]: #method.get_mut
    ///
    /// # Examples
    ///
    /// ```
    /// use halfbrown::HashMap;
    /// use halfbrown::Entry;
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    /// map.entry("poneyland").or_insert(12);
    ///
    /// assert_eq!(map["poneyland"], 12);
    /// if let Entry::Occupied(o) = map.entry("poneyland") {
    ///     *o.into_mut() += 10;
    /// }
    ///
    /// assert_eq!(map["poneyland"], 22);
    /// ```
    #[inline]
    pub fn into_mut(self) -> &'a mut V {
        match self.0 {
            OccupiedEntryInt::Map(m) => m.into_mut(),
            OccupiedEntryInt::Vec(m) => m.into_mut(),
        }
    }

    /// Sets the value of the entry, and returns the entry's old value.
    ///
    /// # Examples
    ///
    /// ```
    /// use halfbrown::HashMap;
    /// use halfbrown::Entry;
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    /// map.entry("poneyland").or_insert(12);
    ///
    /// if let Entry::Occupied(mut o) = map.entry("poneyland") {
    ///     assert_eq!(o.insert(15), 12);
    /// }
    ///
    /// assert_eq!(map["poneyland"], 15);
    /// ```
    #[inline]
    pub fn insert(&mut self, value: V) -> V {
        match &mut self.0 {
            OccupiedEntryInt::Map(m) => m.insert(value),
            OccupiedEntryInt::Vec(m) => m.insert(value),
        }
    }

    /// Takes the value out of the entry, and returns it.
    ///
    /// # Examples
    ///
    /// ```
    /// use halfbrown::HashMap;
    /// use halfbrown::Entry;
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    /// map.entry("poneyland").or_insert(12);
    ///
    /// if let Entry::Occupied(o) = map.entry("poneyland") {
    ///     assert_eq!(o.remove(), 12);
    /// }
    ///
    /// assert_eq!(map.contains_key("poneyland"), false);
    /// ```
    #[inline]
    pub fn remove(self) -> V {
        match self.0 {
            OccupiedEntryInt::Map(m) => m.remove(),
            OccupiedEntryInt::Vec(m) => m.remove(),
        }
    }

    /// Replaces the entry, returning the old key and value. The new key in the hash map will be
    /// the key used to create this entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::hash_map::{Entry, HashMap};
    /// use std::rc::Rc;
    ///
    /// let mut map: HashMap<Rc<String>, u32> = HashMap::new();
    /// map.insert(Rc::new("Stringthing".to_string()), 15);
    ///
    /// let my_key = Rc::new("Stringthing".to_string());
    ///
    /// if let Entry::Occupied(entry) = map.entry(my_key) {
    ///     // Also replace the key with a handle to our other key.
    ///     let (old_key, old_value): (Rc<String>, u32) = entry.replace_entry(16);
    /// }
    ///
    /// ```
    #[inline]
    pub fn replace_entry(self, value: V) -> (K, V) {
        match self.0 {
            OccupiedEntryInt::Map(m) => m.replace_entry(value),
            OccupiedEntryInt::Vec(m) => m.replace_entry(value),
        }
    }

    /// Replaces the key in the hash map with the key used to create this entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::hash_map::{Entry, HashMap};
    /// use std::rc::Rc;
    ///
    /// let mut map: HashMap<Rc<String>, u32> = HashMap::new();
    /// let mut known_strings: Vec<Rc<String>> = Vec::new();
    ///
    /// // Initialise known strings, run program, etc.
    ///
    /// reclaim_memory(&mut map, &known_strings);
    ///
    /// fn reclaim_memory(map: &mut HashMap<Rc<String>, u32>, known_strings: &[Rc<String>] ) {
    ///     for s in known_strings {
    ///         if let Entry::Occupied(entry) = map.entry(s.clone()) {
    ///             // Replaces the entry's key with our version of it in `known_strings`.
    ///             entry.replace_key();
    ///         }
    ///     }
    /// }
    /// ```
    #[inline]
    pub fn replace_key(self) -> K {
        match self.0 {
            OccupiedEntryInt::Map(m) => m.replace_key(),
            OccupiedEntryInt::Vec(m) => m.replace_key(),
        }
    }
}

impl<'a, K, V, S> VacantEntry<'a, K, V, S>
where
    S: BuildHasher,
{
    /// Gets a reference to the key that would be used when inserting a value
    /// through the `VacantEntry`.
    ///
    /// # Examples
    ///
    /// ```
    /// use halfbrown::HashMap;
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    /// assert_eq!(map.entry("poneyland").key(), &"poneyland");
    /// ```
    #[inline]
    pub fn key(&self) -> &K {
        match &self.0 {
            VacantEntryInt::Map(m) => m.key(),
            VacantEntryInt::Vec(m) => m.key(),
        }
    }

    /// Take ownership of the key.
    ///
    /// # Examples
    ///
    /// ```
    /// use halfbrown::HashMap;
    /// use halfbrown::Entry;
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    ///
    /// if let Entry::Vacant(v) = map.entry("poneyland") {
    ///     v.into_key();
    /// }
    /// ```
    #[inline]
    pub fn into_key(self) -> K {
        match self.0 {
            VacantEntryInt::Map(m) => m.into_key(),
            VacantEntryInt::Vec(m) => m.into_key(),
        }
    }

    /// Sets the value of the entry with the `VacantEntry`'s key,
    /// and returns a mutable reference to it.
    ///
    /// # Examples
    ///
    /// ```
    /// use halfbrown::HashMap;
    /// use halfbrown::Entry;
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    ///
    /// if let Entry::Vacant(o) = map.entry("poneyland") {
    ///     o.insert(37);
    /// }
    /// assert_eq!(map["poneyland"], 37);
    /// ```
    #[inline]
    pub fn insert(self, value: V) -> &'a mut V
    where
        K: Hash,
        S: BuildHasher,
    {
        match self.0 {
            VacantEntryInt::Map(m) => m.insert(value),
            VacantEntryInt::Vec(m) => m.insert(value),
        }
    }
}
