use std::{
    alloc::Layout,
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

thread_local! {
    static INTERNED: RefCell<HashMap<TypeId, HashMap<u64, Static>, TypeIdHasherBuilder>> = RefCell::new(HashMap::with_hasher(TypeIdHasherBuilder));
    static MEMOIZED: RefCell<HashMap<TypeId, HashMap<u64, Static>, TypeIdHasherBuilder>> = RefCell::new(HashMap::with_hasher(TypeIdHasherBuilder));
}

pub fn static_type_id<T>() -> TypeId
where
    T: Staticize,
{
    TypeId::of::<T::Static>()
}

pub trait Staticize {
    type Static: 'static;
}

impl<'a, T> Staticize for &'a T
where
    T: Staticize,
{
    type Static = &'static T::Static;
}

#[macro_export]
macro_rules! derive_staticize {
    ($typ:ty) => {
        impl $crate::memoized::Staticize for $typ {
            type Static = &'static $typ;
        }
    };
}

#[macro_export]
macro_rules! derive_staticize_slice {
    (&$typ:ty) => {
        impl $crate::memoized::Staticize for &$typ {
            type Static = &'static $typ;
        }
    };
}

pub trait IsReference {
    type Reference;
}

pub enum True {}
pub enum False {}

impl<'a, T: Copy> IsReference for &'a T {
    type Reference = True;
}

#[macro_export]
macro_rules! impl_is_reference {
    ($typ:ty, True) => {
        impl $crate::memoized::IsReference for $typ {
            type Reference = $crate::memoized::True;
        }
    };
    ($typ:ty, False) => {
        impl $crate::memoized::IsReference for $typ {
            type Reference = $crate::memoized::False;
        }
    };
}

impl_is_reference!(bool, False);
impl_is_reference!(str, False);
impl_is_reference!(String, False);
impl_is_reference!(usize, False);
impl_is_reference!(u8, False);
impl_is_reference!(u16, False);
impl_is_reference!(u32, False);
impl_is_reference!(u64, False);
impl_is_reference!(u128, False);
impl_is_reference!(i8, False);
impl_is_reference!(i16, False);
impl_is_reference!(i32, False);
impl_is_reference!(i64, False);
impl_is_reference!(i128, False);
impl_is_reference!(f32, False);
impl_is_reference!(f64, False);

derive_staticize_slice!(&str);
derive_staticize_slice!(&[u8]);
derive_staticize_slice!(&[char]);

derive_staticize!(bool);
derive_staticize!(str);
derive_staticize!(String);
derive_staticize!(usize);
derive_staticize!(u8);
derive_staticize!(u16);
derive_staticize!(u32);
derive_staticize!(u64);
derive_staticize!(u128);
derive_staticize!(i8);
derive_staticize!(i16);
derive_staticize!(i32);
derive_staticize!(i64);
derive_staticize!(i128);
derive_staticize!(f32);
derive_staticize!(f64);

#[derive(Copy, Clone)]
struct StaticValue {
    ptr: *const (),
    hash: u64,
}

impl StaticValue {
    pub const unsafe fn as_ref<'a, T>(&self) -> &'a T {
        &*(self.ptr as *const T)
    }

    pub fn from<T: Hash>(value: T) -> Self {
        let mut hasher = DefaultHasher::default();
        value.hash(&mut hasher);
        let hash = hasher.finish();
        let ptr = (Box::leak(Box::from(value)) as *const T) as *const ();
        StaticValue { ptr, hash }
    }
}

impl PartialEq for StaticValue {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl Eq for StaticValue {}

impl Hash for StaticValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

impl PartialOrd for StaticValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.hash.partial_cmp(&other.hash)
    }
}

impl Ord for StaticValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hash.cmp(&other.hash)
    }
}

unsafe impl Send for StaticValue {}
unsafe impl Sync for StaticValue {}

impl std::fmt::Debug for StaticValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StaticAlloc")
            .field("hash", &self.hash)
            .finish()
    }
}

#[derive(Copy, Clone)]
struct StaticSlice {
    ptr: *const (),
    len: usize,
    hash: u64,
}

impl StaticSlice {
    pub const unsafe fn as_slice<'a, T>(&self) -> &'a [T] {
        std::slice::from_raw_parts(self.ptr as *const T, self.len)
    }

    pub fn from<T: Hash + Copy>(slice: &[T]) -> Self {
        let mut hasher = DefaultHasher::default();
        slice.hash(&mut hasher);
        let hash = hasher.finish();
        let ptr = unsafe {
            let ptr = std::alloc::alloc(Layout::array::<T>(slice.len()).unwrap()) as *mut T;
            std::ptr::copy(slice.as_ptr(), ptr, slice.len());
            ptr
        };
        let ptr = (ptr as *const T) as *const ();
        let len = slice.len();
        StaticSlice { ptr, len, hash }
    }
}

impl Hash for StaticSlice {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

impl PartialEq for StaticSlice {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl Eq for StaticSlice {}

impl PartialOrd for StaticSlice {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.hash.partial_cmp(&other.hash)
    }
}

impl Ord for StaticSlice {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hash.cmp(&other.hash)
    }
}

unsafe impl Send for StaticSlice {}
unsafe impl Sync for StaticSlice {}

impl std::fmt::Debug for StaticSlice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SliceAlloc")
            .field("hash", &self.hash)
            .finish()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Static {
    Value(StaticValue),
    Slice(StaticSlice),
}

impl<T: Hash + Copy> From<&[T]> for Static {
    fn from(slice: &[T]) -> Self {
        Static::Slice(StaticSlice::from(slice))
    }
}

impl Static {
    pub fn from_value<T: Hash + Copy>(value: T) -> Static {
        Static::Value(StaticValue::from(value))
    }

    pub const unsafe fn as_slice<'a, T>(&self) -> &'a [T] {
        match self {
            Static::Value(_) => panic!("not a slice type!"),
            Static::Slice(static_slice) => static_slice.as_slice(),
        }
    }

    pub const unsafe fn as_ref<'a, T>(&self) -> &'a T {
        match self {
            Static::Value(static_value) => static_value.as_ref(),
            Static::Slice(_) => panic!("not a value type!"),
        }
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

#[derive(Copy, Clone)]
pub struct Interned<T: Hash> {
    _value: PhantomData<T>,
    value: Static,
}

impl<T: Hash + Copy> From<&[T]> for Interned<T>
where
    for<'a> &'a [T]: Staticize + IsReference<Reference = True>,
{
    fn from(slice: &[T]) -> Self {
        let mut hasher = DefaultHasher::default();
        slice.hash(&mut hasher);
        let hash = hasher.finish();
        let type_id = static_type_id::<&[T]>();
        let entry = INTERNED.with(|interned| {
            *interned
                .borrow_mut()
                .entry(type_id)
                .or_insert_with(|| HashMap::new())
                .entry(hash)
                .or_insert_with(|| Static::Slice(StaticSlice::from(slice)))
        });
        Interned {
            _value: PhantomData,
            value: entry,
        }
    }
}

impl<T: Hash + Staticize + IsReference<Reference = False>> Interned<T> {
    pub fn from_value(value: T) -> Self {
        let mut hasher = DefaultHasher::default();
        value.hash(&mut hasher);
        let hash = hasher.finish();
        let type_id = static_type_id::<T>();
        let entry = INTERNED.with(|interned| {
            *interned
                .borrow_mut()
                .entry(type_id)
                .or_insert_with(|| HashMap::new())
                .entry(hash)
                .or_insert_with(|| Static::Value(StaticValue::from(value)))
        });
        Interned {
            _value: PhantomData,
            value: entry,
        }
    }

    pub fn interned_value<'a>(&self) -> &'a T {
        unsafe { self.value.as_ref() }
    }

    pub fn interned_value_copy(&self) -> T {
        let copy: T = unsafe { std::mem::transmute_copy(self.interned_value()) };
        copy
    }
}

impl<T: Hash + Staticize> Deref for Interned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.interned_value()
    }
}

impl<T: Hash + PartialEq + Staticize> PartialEq for Interned<T> {
    fn eq(&self, other: &Self) -> bool {
        self.interned_value() == other.interned_value()
    }
}

impl<T: Hash + Staticize + Eq> Eq for Interned<T> {}

impl<T: Hash + Staticize + PartialOrd> PartialOrd for Interned<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.interned_value().partial_cmp(other.interned_value())
    }
}

impl<T: Hash + Staticize + Ord> Ord for Interned<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.interned_value().cmp(other.interned_value())
    }
}

impl<T: Hash + Staticize> Hash for Interned<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.interned_value().hash(state)
    }
}

impl<T: Hash + Staticize + std::fmt::Debug> std::fmt::Debug for Interned<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Interned")
            .field("interned_value", &self.interned_value())
            .finish()
    }
}

impl<T: Hash + Staticize + Display> Display for Interned<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.interned_value().fmt(f)
    }
}

#[derive(Copy, Clone)]
pub struct Memoized<I: Hash, T: Hash + Staticize> {
    _input: PhantomData<I>,
    interned: Interned<T>,
}

impl<I: Hash, T: Hash + Staticize> Memoized<I, T> {
    pub fn from<G>(input: &I, generator: G) -> Self
    where
        G: Fn(&I) -> T,
    {
        let mut hasher = DefaultHasher::default();
        input.hash(&mut hasher);
        let input_hash = hasher.finish();
        let type_id = static_type_id::<T>();
        let generate_value = || -> StaticValue { StaticValue::from(generator(input)) };
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
        let value: T = unsafe { std::mem::transmute_copy(entry.as_ref::<T>()) };
        Memoized {
            _input: PhantomData,
            interned: Interned::from(value),
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

impl<I: Hash, T: Hash + Staticize> Deref for Memoized<I, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.interned_value()
    }
}

impl<I: Hash, T: Hash + PartialEq + Staticize> PartialEq for Memoized<I, T> {
    fn eq(&self, other: &Self) -> bool {
        self.interned_value() == other.interned_value()
    }
}

impl<I: Hash, T: Hash + Eq + Staticize> Eq for Memoized<I, T> {}

impl<I: Hash, T: Hash + PartialOrd + Staticize> PartialOrd for Memoized<I, T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.interned_value().partial_cmp(other.interned_value())
    }
}

impl<I: Hash, T: Hash + Ord + Staticize> Ord for Memoized<I, T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.interned_value().cmp(other.interned_value())
    }
}

impl<I: Hash, T: Hash + Staticize> Hash for Memoized<I, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.interned_value().hash(state)
    }
}

impl<I: Hash, T: Hash + Staticize + std::fmt::Debug> std::fmt::Debug for Memoized<I, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Memoized")
            .field("interned_value", &self.interned_value())
            .finish()
    }
}

impl<I: Hash, T: Hash + Staticize + Display> Display for Memoized<I, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.interned_value().fmt(f)
    }
}

#[test]
fn test_static_alloc() {
    let a = StaticValue::from(37);
    assert_eq!(unsafe { *a.as_ref::<i32>() }, 37);
    let b = StaticValue::from(37);
    assert_eq!(a, b); // note: we base equality off of the hash, not the address
    let c = StaticValue::from(8348783947u64);
    assert_ne!(b, c);
    assert_eq!(unsafe { *c.as_ref::<u64>() }, 8348783947u64);
    let d = StaticValue::from(String::from("test"));
    assert_eq!(unsafe { d.as_ref::<String>() }, &"test");
    let e = StaticValue::from("test");
    assert_eq!(unsafe { e.as_ref::<&str>() }, &"test");
}

#[cfg(test)]
fn num_memoized<T: Staticize>() -> usize {
    let type_id = static_type_id::<T>();
    MEMOIZED.with(|interned| interned.borrow_mut().entry(type_id).or_default().len())
}

#[cfg(test)]
fn num_interned<T: Staticize>() -> usize {
    let type_id = static_type_id::<T>();
    INTERNED.with(|interned| interned.borrow_mut().entry(type_id).or_default().len())
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
    println!(
        "{}",
        INTERNED.with(|interned| format!("{:?}", interned.borrow()))
    );
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
    let st = String::from("asdf");
    let c = Interned::from(st.as_str());
    let st2 = String::from("asdf");
    let d = Interned::from(st2.as_str());
    assert_eq!(c, d);
    assert_ne!(c, b);
    let st3 = String::from("nope nope");
    let e = Interned::from(st3.as_str());
    assert_ne!(d, e);
    assert_eq!(c.interned_value().as_ptr(), d.interned_value().as_ptr());
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
    assert_eq!(num_interned::<usize>(), initial_interned + 2);
    assert_eq!(num_memoized::<usize>(), initial_memoized + 2);
}

#[test]
fn test_interned_byte_arrays() {
    let a = Interned::from([1u8, 2u8, 3u8].as_slice());
    let b = Interned::from([5u8, 4u8, 3u8, 2u8, 1u8].as_slice());
    assert_ne!(a.interned_value().as_ptr(), b.interned_value().as_ptr());
    let c = Interned::from([1u8, 2u8, 3u8].as_slice());
    assert_eq!(a.interned_value().as_ptr(), c.interned_value().as_ptr());
    assert_eq!(a.interned_value(), c.interned_value());
    assert_eq!(a, c);
}

#[test]
fn test_static_slice_lifetimes() {
    let slice = &mut [1, 2, 3, 4, 5];
    let a = StaticSlice::from(slice);
    assert_eq!(unsafe { a.as_slice::<i32>() }, &[1, 2, 3, 4, 5]);
    slice[1] = 7;
    assert_eq!(unsafe { a.as_slice::<i32>() }, [1, 2, 3, 4, 5]);
    let b = StaticSlice::from(&[1, 2, 3, 4, 5]);
    assert_eq!(a, b);
    let c = StaticSlice::from(&[true, false, true, false, true, false]);
    assert_ne!(a, c);
    assert_ne!(b, c);
    assert_eq!(
        unsafe { c.as_slice::<bool>() },
        &[true, false, true, false, true, false]
    );
}
