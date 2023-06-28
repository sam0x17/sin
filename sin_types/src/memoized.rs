use std::{
    any::TypeId,
    cell::RefCell,
    collections::{
        hash_map::{DefaultHasher, Entry},
        HashMap,
    },
    fmt::Display,
    hash::{BuildHasher, Hash, Hasher},
    marker::PhantomData,
    ops::Deref,
};

/// implementation detail
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct StaticAlloc(*const ());

impl StaticAlloc {
    pub const unsafe fn as_ref<'a, T>(&self) -> &'a T {
        &*(self.0 as *const T)
    }

    pub fn from<T>(value: T) -> Self {
        StaticAlloc((Box::leak(Box::from(value)) as *const T) as *const ())
    }
}

struct TypeIdHasher {
    hash: Option<u64>,
}

impl Hasher for TypeIdHasher {
    fn finish(&self) -> u64 {
        self.hash.unwrap()
    }

    fn write(&mut self, bytes: &[u8]) {
        debug_assert!(bytes.len() == 8);
        self.hash = Some(bytes.as_ptr() as u64);
    }
}

struct TypeIdHasherBuilder;

impl BuildHasher for TypeIdHasherBuilder {
    type Hasher = TypeIdHasher;

    fn build_hasher(&self) -> Self::Hasher {
        TypeIdHasher { hash: None }
    }
}

thread_local! {
    static INTERNED: RefCell<HashMap<TypeId, HashMap<u64, StaticAlloc>, TypeIdHasherBuilder>> = RefCell::new(HashMap::with_hasher(TypeIdHasherBuilder));
    static MEMOIZED: RefCell<HashMap<TypeId, HashMap<u64, StaticAlloc>, TypeIdHasherBuilder>> = RefCell::new(HashMap::with_hasher(TypeIdHasherBuilder));
}

pub struct Interned<T: Hash> {
    _value: PhantomData<T>,
    value: StaticAlloc,
}

impl<T: Hash> Clone for Interned<T> {
    fn clone(&self) -> Self {
        Self {
            _value: self._value.clone(),
            value: self.value.clone(),
        }
    }
}

impl<T: Hash> Copy for Interned<T> {}

impl<T: Hash> Interned<T> {
    pub fn from(value: T) -> Self
    where
        T: 'static,
    {
        let mut hasher = DefaultHasher::default();
        value.hash(&mut hasher);
        let hash = hasher.finish();
        let type_id = TypeId::of::<T>();
        let entry = INTERNED.with(|interned| {
            *interned
                .borrow_mut()
                .entry(type_id)
                .or_insert_with(|| HashMap::new())
                .entry(hash)
                .or_insert_with(|| StaticAlloc::from(value))
        });
        Interned {
            _value: PhantomData,
            value: entry,
        }
    }

    pub fn interned_value(&self) -> &T {
        unsafe { self.value.as_ref() }
    }
}

impl<T: Hash> Deref for Interned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.interned_value()
    }
}

impl<T: Hash + PartialEq> PartialEq for Interned<T> {
    fn eq(&self, other: &Self) -> bool {
        self.interned_value() == other.interned_value()
    }
}

impl<T: Hash + Eq> Eq for Interned<T> {}

impl<T: Hash + PartialOrd> PartialOrd for Interned<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.interned_value().partial_cmp(other.interned_value())
    }
}

impl<T: Hash + Ord> Ord for Interned<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.interned_value().cmp(other.interned_value())
    }
}

impl<T: Hash> Hash for Interned<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.interned_value().hash(state)
    }
}

impl<T: Hash + std::fmt::Debug> std::fmt::Debug for Interned<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Interned")
            .field("interned_value", &self.interned_value())
            .finish()
    }
}

impl<T: Hash + Display> Display for Interned<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.interned_value().fmt(f)
    }
}

pub struct Memoized<I: Hash, T: Hash> {
    _input: PhantomData<I>,
    interned: Interned<T>,
}

impl<I: Hash, T: Hash + Clone> Memoized<I, T> {
    pub fn from<G>(input: &I, generator: G) -> Self
    where
        G: Fn(&I) -> T,
        T: 'static,
    {
        let mut hasher = DefaultHasher::default();
        input.hash(&mut hasher);
        let input_hash = hasher.finish();
        let type_id = TypeId::of::<T>();
        let generate_value = || -> StaticAlloc { StaticAlloc::from(generator(input)) };
        let entry = MEMOIZED.with(|memoized| {
            match (*memoized)
                .borrow_mut()
                .entry(type_id)
                .or_insert_with(|| HashMap::new())
                .entry(input_hash)
            {
                Entry::Occupied(entry) => return *entry.get(), // break early
                Entry::Vacant(entry) => *entry.insert(generate_value()),
            }
        });
        Memoized {
            _input: PhantomData,
            interned: Interned::from(unsafe { entry.as_ref::<T>().clone() }),
        }
    }

    pub fn interned_value(&self) -> &T {
        self.interned.interned_value()
    }

    pub fn interned(&self) -> Interned<T> {
        Interned {
            _value: PhantomData,
            value: self.interned.value,
        }
    }
}

impl<I: Hash, T: Hash> Clone for Memoized<I, T> {
    fn clone(&self) -> Self {
        Self {
            _input: self._input.clone(),
            interned: self.interned.clone(),
        }
    }
}

impl<I: Hash, T: Hash> Copy for Memoized<I, T> {}

impl<I: Hash, T: Hash + Clone> Deref for Memoized<I, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.interned_value()
    }
}

impl<I: Hash, T: Hash + Clone + PartialEq> PartialEq for Memoized<I, T> {
    fn eq(&self, other: &Self) -> bool {
        self.interned_value() == other.interned_value()
    }
}

impl<I: Hash, T: Hash + Clone + Eq> Eq for Memoized<I, T> {}

impl<I: Hash, T: Hash + Clone + PartialOrd> PartialOrd for Memoized<I, T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.interned_value().partial_cmp(other.interned_value())
    }
}

impl<I: Hash, T: Hash + Clone + Ord> Ord for Memoized<I, T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.interned_value().cmp(other.interned_value())
    }
}

impl<I: Hash, T: Hash + Clone> Hash for Memoized<I, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.interned_value().hash(state)
    }
}

impl<I: Hash, T: Hash + Clone + std::fmt::Debug> std::fmt::Debug for Memoized<I, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Memoized")
            .field("interned_value", &self.interned_value())
            .finish()
    }
}

impl<I: Hash, T: Hash + Clone + Display> Display for Memoized<I, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.interned_value().fmt(f)
    }
}

#[test]
fn test_static_alloc() {
    let a = StaticAlloc::from(37);
    assert_eq!(unsafe { *a.as_ref::<i32>() }, 37);
    let b = StaticAlloc::from(37);
    assert_ne!(a, b);
    let c = StaticAlloc::from(8348783947u64);
    assert_eq!(unsafe { *c.as_ref::<u64>() }, 8348783947u64);
    let d = StaticAlloc::from(String::from("test"));
    assert_eq!(unsafe { d.as_ref::<String>() }, &"test");
    let e = StaticAlloc::from("test");
    assert_eq!(unsafe { e.as_ref::<&str>() }, &"test");
}

#[cfg(test)]
fn num_memoized<T: 'static>() -> usize {
    MEMOIZED.with(|interned| {
        interned
            .borrow_mut()
            .entry(TypeId::of::<T>())
            .or_default()
            .len()
    })
}

#[cfg(test)]
fn num_interned<T: 'static>() -> usize {
    INTERNED.with(|interned| {
        interned
            .borrow_mut()
            .entry(TypeId::of::<T>())
            .or_default()
            .len()
    })
}

#[test]
fn test_interned_basics() {
    let initial_interned = num_interned::<i32>();
    let a = Interned::from(32);
    let b = Interned::from(27);
    assert_ne!(a, b);
    let c = Interned::from(32);
    assert_eq!(a, c);
    assert_ne!(b, c);
    assert_eq!(*a.interned_value(), 32);
    assert_eq!(*b.interned_value(), 27);
    assert_eq!(*c.interned_value(), 32);
    assert_eq!(num_interned::<i32>(), initial_interned + 2);
}

#[test]
fn test_interned_str_types() {
    let a = Interned::from("this is a triumph");
    let b = Interned::from("I'm making a note here: huge success");
    assert_ne!(a, b);
    assert_ne!(a.interned_value(), b.interned_value());
    assert_eq!(a.interned_value(), &"this is a triumph");
    assert_eq!(b.interned_value(), &"I'm making a note here: huge success");
}

#[test]
fn test_interned_deref() {
    let c = Interned::from("for the good of all of us except the ones who are dead");
    assert_eq!(c.chars().next().unwrap(), 'f');
}

#[test]
fn test_memoized_basic() {
    let initial_interned = num_interned::<usize>();
    let initial_memoized = num_memoized::<usize>();
    let a = Memoized::from(&"some_input", |input| input.len());
    let b = Memoized::from(&"other", |input| input.len());
    assert_ne!(a, b);
    let c = Memoized::from(&"some_input", |input| input.len());
    assert_eq!(a, c);
    assert_ne!(b, c);
    assert_eq!(*a.interned_value(), 10);
    assert_eq!(*b.interned_value(), 5);
    assert_eq!(*c.interned_value(), 10);
    assert_eq!(num_memoized::<usize>(), initial_memoized + 2);
    assert_eq!(num_interned::<usize>(), initial_interned + 2);
}
