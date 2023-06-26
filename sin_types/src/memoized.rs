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

pub struct Interned<I: Hash, T: Hash + Sync> {
    _input: PhantomData<I>,
    _value: PhantomData<T>,
    value: StaticAlloc,
}

impl<I: Hash, T: Hash + Sync> Interned<I, T> {
    pub fn memoize<R: AsRef<I>, G>(input: R, generator: G) -> Self
    where
        G: Fn(&I) -> T,
    {
        let input = input.as_ref();
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
        let mut hasher = DefaultHasher::default();
        unsafe { entry.as_ref::<T>().hash(&mut hasher) };
        let value_hash = hasher.finish();
        let entry =
            INTERNED.with(|interned| *interned.borrow_mut().entry(value_hash).or_insert(entry));
        Interned {
            _input: PhantomData,
            _value: PhantomData,
            value: entry,
        }
    }

    pub fn from<R: AsRef<T>>(value: R) -> Self {
        let value = value.as_ref();
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
            _input: PhantomData,
            _value: PhantomData,
            value: entry,
        }
    }

    pub fn interned_value(&self) -> &T {
        unsafe { self.value.as_ref() }
    }
}

impl<I: Hash, T: Hash + Sync> Deref for Interned<I, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.interned_value()
    }
}
