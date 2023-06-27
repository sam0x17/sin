use std::{
    cell::RefCell,
    collections::{
        hash_map::{DefaultHasher, Entry},
        HashMap,
    },
    fmt::Display,
    hash::{Hash, Hasher},
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

thread_local! {
    static MEMOIZED: RefCell<HashMap<u64, StaticAlloc>> = RefCell::new(HashMap::new());
    static INTERNED: RefCell<HashMap<u64, StaticAlloc>> = RefCell::new(HashMap::new());
}

#[derive(Copy, Clone)]
pub struct Interned<T: Hash> {
    _value: PhantomData<T>,
    value: StaticAlloc,
}

impl<T: Hash> Interned<T> {
    pub fn from(value: T) -> Self {
        let mut hasher = DefaultHasher::default();
        value.hash(&mut hasher);
        let hash = hasher.finish();
        let entry = INTERNED.with(|interned| {
            *interned
                .borrow_mut()
                .entry(hash)
                .or_insert(StaticAlloc::from(value))
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
    pub fn memoize<G>(input: &I, generator: G) -> Self
    where
        G: Fn(&I) -> T,
    {
        // TODO: UB in event of hash collision
        // TODO: prefix keys with type to avoid multi-type collision
        let mut hasher = DefaultHasher::default();
        input.hash(&mut hasher);
        let input_hash = hasher.finish();
        let generate_value = || -> StaticAlloc { StaticAlloc::from(generator(input)) };
        let entry = MEMOIZED.with(
            |memoized| match (*memoized).borrow_mut().entry(input_hash) {
                Entry::Occupied(entry) => return *entry.get(), // break early
                Entry::Vacant(entry) => *entry.insert(generate_value()),
            },
        );
        // only check INTERNED if MEMOIZED didn't have the entry
        let value: &T = unsafe { entry.as_ref::<T>() };
        Memoized {
            _input: PhantomData,
            interned: Interned::from(value.clone()),
        }
    }

    pub fn interned_value(&self) -> &T {
        self.interned.interned_value()
    }
}

impl<I: Hash, T: Hash + Clone> Deref for Memoized<I, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.interned_value()
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
fn num_memoized() -> usize {
    MEMOIZED.with(|memoized| memoized.borrow().len())
}

#[cfg(test)]
fn num_interned() -> usize {
    INTERNED.with(|interned| interned.borrow().len())
}

#[test]
fn test_interned_basics() {
    let initial_interned = num_interned();
    let a = Interned::from(32);
    let b = Interned::from(27);
    assert_ne!(a, b);
    let c = Interned::from(32);
    assert_eq!(a, c);
    assert_ne!(b, c);
    assert_eq!(*a.interned_value(), 32);
    assert_eq!(*b.interned_value(), 27);
    assert_eq!(*c.interned_value(), 32);
    assert_eq!(num_interned(), initial_interned + 2);
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
