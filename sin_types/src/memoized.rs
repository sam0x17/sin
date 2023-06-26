use std::{
    cell::RefCell,
    collections::{
        hash_map::{DefaultHasher, Entry},
        HashMap,
    },
    hash::{Hash, Hasher},
    marker::PhantomData,
    ops::Deref,
};

/// implementation detail
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
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

pub struct Interned<T: Hash + Sync> {
    _value: PhantomData<T>,
    value: StaticAlloc,
}

impl<T: Hash + Sync> Interned<T> {
    pub fn from(value: &T) -> Self {
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

impl<T: Hash + Sync> Deref for Interned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.interned_value()
    }
}

pub struct Memoized<I: Hash, T: Hash + Sync> {
    _input: PhantomData<I>,
    interned: Interned<T>,
}

impl<I: Hash, T: Hash + Sync> Memoized<I, T> {
    pub fn memoize<G>(input: &I, generator: G) -> Self
    where
        G: Fn(&I) -> T,
    {
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
            interned: Interned::from(&value),
        }
    }

    pub fn interned_value(&self) -> &T {
        self.interned.interned_value()
    }
}

impl<I: Hash, T: Hash + Sync> Deref for Memoized<I, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.interned_value()
    }
}
