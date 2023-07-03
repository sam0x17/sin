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

pub fn static_type_id<T: Staticize>() -> TypeId {
    TypeId::of::<T::Static>()
}

pub fn static_type_name<T: Staticize>() -> &'static str {
    &std::any::type_name::<T::Static>()
}

pub trait Staticize {
    type Static: 'static + ?Sized;
}

impl<'a, T: ?Sized> Staticize for &'a T
where
    T: Staticize,
{
    type Static = &'static T::Static;
}

impl<'a, T: Staticize> Staticize for &'a [T]
where
    <T as Staticize>::Static: Sized,
{
    type Static = &'static [T::Static];
}

#[macro_export]
macro_rules! derive_staticize {
    ($typ:ty) => {
        impl $crate::memoized::Staticize for $typ {
            type Static = $typ;
        }
    };
}

pub trait DataType {
    type Type;
    type SliceType;
    type ReferenceType;
    type ValueType;
    type SliceValueType;
    type ReferenceValueType: ?Sized;
    type InnerType: ?Sized;
    type DerefType;

    fn as_slice(&self) -> &[Self::SliceValueType];
    fn as_value(&self) -> Self::ValueType;
    fn to_static_with_hash(&self, hash: Option<u64>) -> Static;

    fn to_static(&self) -> Static {
        self.to_static_with_hash(None)
    }
}

pub enum Slice {}
pub enum Reference {}
pub enum Value {}

impl<'a, T: Sized + Hash + Copy> DataType for &'a [T] {
    type Type = Slice;
    type SliceType = &'a [T];
    type ReferenceType = Self::SliceType;
    type ValueType = Self::SliceType;
    type SliceValueType = T;
    type ReferenceValueType = [T];
    type InnerType = T;
    type DerefType = &'a [T];

    fn as_slice(&self) -> &'a [T] {
        *self
    }

    fn as_value(&self) -> &'a [T] {
        *self
    }

    fn to_static_with_hash(&self, hash: Option<u64>) -> Static {
        Static::from(*self, hash)
    }
}

#[macro_export]
macro_rules! impl_data_type {
    ($typ:ty, Value) => {
        impl $crate::memoized::DataType for $typ {
            type Type = $crate::memoized::Value;
            type SliceType = ();
            type ReferenceType = ();
            type ValueType = $typ;
            type SliceValueType = ();
            type ReferenceValueType = ();
            type InnerType = $typ;
            type DerefType = $typ;

            fn as_slice(&self) -> &'static [Self::SliceType] {
                panic!("not a slice!");
            }

            fn as_value(&self) -> Self::ValueType {
                *self
            }

            fn to_static_with_hash(&self, hash: Option<u64>) -> Static {
                Static::from_value(*self, hash)
            }
        }
    };
}

impl<'a> DataType for &'a str {
    type Type = Reference;
    type SliceType = &'a str;
    type ReferenceType = &'a str;
    type ValueType = &'a str;
    type SliceValueType = ();
    type ReferenceValueType = str;
    type InnerType = str;
    type DerefType = &'a str;

    fn as_slice(&self) -> &'static [()] {
        panic!("not supported");
    }

    fn as_value(&self) -> &'a str {
        *self
    }

    fn to_static_with_hash(&self, hash: Option<u64>) -> Static {
        Static::from_str(*self, hash)
    }
}

impl_data_type!(bool, Value);
impl_data_type!(usize, Value);
impl_data_type!(u8, Value);
impl_data_type!(u16, Value);
impl_data_type!(u32, Value);
impl_data_type!(u64, Value);
impl_data_type!(u128, Value);
impl_data_type!(i8, Value);
impl_data_type!(i16, Value);
impl_data_type!(i32, Value);
impl_data_type!(i64, Value);
impl_data_type!(i128, Value);

derive_staticize!(str);
derive_staticize!(bool);
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
pub struct StaticValue {
    ptr: *const (),
    hash: u64,
}

impl StaticValue {
    pub const unsafe fn as_value<'a, T>(&self) -> &'a T {
        &*(self.ptr as *const T)
    }

    pub fn from<T: Hash>(value: T) -> Self {
        Self::with_hash(value, None)
    }

    pub fn with_hash<T: Hash>(value: T, hash: Option<u64>) -> Self {
        let hash = hash.unwrap_or_else(|| {
            let mut hasher = DefaultHasher::default();
            value.hash(&mut hasher);
            hasher.finish()
        });
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
        f.debug_struct("StaticValue")
            .field("hash", &self.hash)
            .finish()
    }
}

#[derive(Copy, Clone)]
pub struct StaticSlice {
    ptr: *const (),
    len: usize,
    hash: u64,
}

impl StaticSlice {
    pub unsafe fn as_slice<'a, T>(&self) -> &'a [T] {
        std::slice::from_raw_parts(self.ptr as *const T, self.len)
    }

    pub fn from<T: Hash + Copy>(slice: &[T]) -> Self {
        Self::with_hash(slice, None)
    }

    pub fn with_hash<T: Hash + Copy>(slice: &[T], hash: Option<u64>) -> Self {
        let hash = hash.unwrap_or_else(|| {
            let mut hasher = DefaultHasher::default();
            slice.hash(&mut hasher);
            hasher.finish()
        });
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
        f.debug_struct("StaticSlice")
            .field("hash", &self.hash)
            .finish()
    }
}

#[derive(Copy, Clone)]
pub struct StaticStr {
    ptr: *const str,
    hash: u64,
}

impl StaticStr {
    pub const unsafe fn as_str<'a>(&self) -> &'a str {
        &*(self.ptr as *const str)
    }

    pub fn from<T: Hash + Copy>(value: &str) -> Self {
        Self::with_hash(value, None)
    }

    pub fn with_hash(value: &str, hash: Option<u64>) -> Self {
        let hash = hash.unwrap_or_else(|| {
            let mut hasher = DefaultHasher::default();
            value.hash(&mut hasher);
            hasher.finish()
        });
        let ptr = Box::leak(Box::from(value)) as *const str;
        let written_value = unsafe { (ptr as *const str).as_ref().unwrap() };
        assert_eq!(written_value, value);
        StaticStr { ptr, hash }
    }
}

impl Hash for StaticStr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

impl PartialEq for StaticStr {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl Eq for StaticStr {}

impl PartialOrd for StaticStr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.hash.partial_cmp(&other.hash)
    }
}

impl Ord for StaticStr {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hash.cmp(&other.hash)
    }
}

unsafe impl Send for StaticStr {}
unsafe impl Sync for StaticStr {}

impl std::fmt::Debug for StaticStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StaticStr")
            .field("hash", &self.hash)
            .finish()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Static {
    Value(StaticValue),
    Slice(StaticSlice),
    Str(StaticStr),
}

impl Static {
    pub fn as_ptr(&self) -> *const () {
        match self {
            Static::Value(value) => value.ptr,
            Static::Slice(slice) => slice.ptr,
            Static::Str(string) => string.ptr as *const (),
        }
    }

    pub fn hash_code(&self) -> u64 {
        match self {
            Static::Value(value) => value.hash,
            Static::Slice(slice) => slice.hash,
            Static::Str(string) => string.hash,
        }
    }

    fn from<T: Hash + Copy>(slice: &[T], hash: Option<u64>) -> Self {
        Static::Slice(StaticSlice::with_hash(slice, hash))
    }

    pub fn from_value<T: Hash>(value: T, hash: Option<u64>) -> Static {
        Static::Value(StaticValue::with_hash(value, hash))
    }

    pub fn from_str(value: &str, hash: Option<u64>) -> Static {
        Static::Str(StaticStr::with_hash(value, hash))
    }

    pub unsafe fn as_slice<'a, T>(&self) -> &'a [T] {
        match self {
            Static::Slice(static_slice) => static_slice.as_slice::<T>(),
            _ => panic!("not a slice type!"),
        }
    }

    pub unsafe fn as_value<'a, T>(&self) -> &'a T {
        match self {
            Static::Value(static_value) => static_value.as_value::<T>(),
            _ => panic!("not a value type!"),
        }
    }

    pub unsafe fn as_str<'a>(&self) -> &'a str {
        match self {
            Static::Str(static_str) => static_str.as_str(),
            _ => panic!("not a &str!"),
        }
    }

    pub unsafe fn _partial_eq<T: PartialEq + DataType>(&self, other: &Static) -> bool
    where
        T::SliceValueType: PartialEq,
    {
        match (self, other) {
            (Static::Value(a), Static::Value(b)) => *a.as_value::<T>() == *b.as_value::<T>(),
            (Static::Slice(a), Static::Slice(b)) => {
                a.as_slice::<T::SliceValueType>() == b.as_slice::<T::SliceValueType>()
            }
            (Static::Str(a), Static::Str(b)) => a.as_str() == b.as_str(),
            _ => false,
        }
    }

    pub unsafe fn _partial_cmp<T: PartialOrd + Staticize>(
        &self,
        other: &Self,
    ) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Static::Value(a), Static::Value(b)) => {
                a.as_value::<T>().partial_cmp(b.as_value::<T>())
            }
            (Static::Slice(a), Static::Slice(b)) => {
                a.as_slice::<T>().partial_cmp(b.as_slice::<T>())
            }
            (Static::Str(a), Static::Str(b)) => a.as_str().partial_cmp(b.as_str()),
            _ => (static_type_id::<T>(), self.hash_code())
                .partial_cmp(&(static_type_id::<T>(), other.hash_code())),
        }
    }

    pub unsafe fn _cmp<T: Ord + Staticize>(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Static::Value(a), Static::Value(b)) => a.as_value::<T>().cmp(b.as_value::<T>()),
            (Static::Slice(a), Static::Slice(b)) => a.as_slice::<T>().cmp(b.as_slice::<T>()),
            (Static::Str(a), Static::Str(b)) => a.as_str().cmp(b.as_str()),
            _ => (static_type_id::<T>(), self.hash_code())
                .cmp(&(static_type_id::<T>(), other.hash_code())),
        }
    }

    pub unsafe fn _hash<T: Hash + Staticize, H: Hasher>(&self, state: &mut H) {
        let type_id = static_type_id::<T>();
        match self {
            Static::Value(value) => (type_id, value).hash(state),
            Static::Slice(slice) => (type_id, slice).hash(state),
            Static::Str(string) => (type_id, string).hash(state),
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

impl<T: Hash> Interned<T> {
    pub fn as_ptr(&self) -> *const () {
        self.value.as_ptr()
    }
}

impl<T: Hash + Copy + Staticize + DataType> From<Static> for Interned<T> {
    fn from(value: Static) -> Self {
        let type_id = static_type_id::<T>();
        let entry = INTERNED.with(|interned| {
            *interned
                .borrow_mut()
                .entry(type_id)
                .or_insert_with(|| HashMap::new())
                .entry(value.hash_code())
                .or_insert(value)
        });
        Interned {
            _value: PhantomData,
            value: entry,
        }
    }
}

impl<T: Hash + Copy + Staticize + DataType> From<T> for Interned<T::Static>
where
    <T as Staticize>::Static: Hash + Sized,
{
    fn from(value: T) -> Interned<T::Static> {
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
                .or_insert_with(|| value.to_static_with_hash(Some(hash)))
        });
        Interned {
            _value: PhantomData,
            value: entry,
        }
    }
}

impl<T: Hash + Staticize + DataType<Type = Slice>> Interned<T> {
    pub fn interned_slice<'a>(&self) -> &'a [T::SliceValueType] {
        unsafe { self.value.as_slice::<T::SliceValueType>() }
    }
}

impl Interned<&str> {
    pub fn interned_str<'a>(&self) -> &'a str {
        unsafe { self.value.as_str() }
    }
}

impl<T: Hash + Staticize + DataType<Type = Value>> Interned<T> {
    pub fn interned_value<'a>(&self) -> &'a T {
        unsafe { self.value.as_value() }
    }
}

impl<T: Hash + Staticize + DataType<Type = Slice>> Deref for Interned<T> {
    type Target = [T::SliceValueType];

    fn deref(&self) -> &Self::Target {
        match self.value {
            Static::Slice(static_slice) => unsafe { static_slice.as_slice() },
            _ => unreachable!(),
        }
    }
}

impl<T: Hash + PartialEq + Staticize + DataType> PartialEq for Interned<T>
where
    <T as DataType>::SliceValueType: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.value._partial_eq::<T>(&other.value) }
    }
}

impl<T: Hash + Staticize + Eq + DataType> Eq for Interned<T>
where
    T: PartialEq,
    <T as DataType>::SliceValueType: PartialEq,
{
}

impl<T: Hash + Staticize + PartialOrd + DataType> PartialOrd for Interned<T>
where
    <T as DataType>::SliceValueType: PartialEq,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        unsafe { self.value._partial_cmp::<T>(&other.value) }
    }
}

impl<T: Hash + Staticize + Ord + DataType> Ord for Interned<T>
where
    <T as DataType>::SliceValueType: PartialEq,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { self.value._cmp::<T>(&other.value) }
    }
}

impl<T: Hash + Staticize> Hash for Interned<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe { self.value._hash::<T, H>(state) }
    }
}

impl<T: Hash + Staticize + std::fmt::Debug> std::fmt::Debug for Interned<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct(format!("Interned<{}>", static_type_name::<T>()).as_str());
        match self.value {
            Static::Value(value) => f.field("value", unsafe { value.as_value::<T>() }),
            Static::Slice(slice) => f.field("slice", unsafe { &slice.as_slice::<T>() }),
            Static::Str(string) => f.field("str", unsafe { &string.as_str() }),
        }
        .finish()
    }
}

impl<T: Hash + Display> Display for Interned<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Pointer;
        match self.value {
            Static::Value(value) => unsafe { value.as_value::<T>().fmt(f) },
            Static::Slice(slice) => unsafe { slice.as_slice::<T>().fmt(f) },
            Static::Str(string) => unsafe { string.as_str().fmt(f) },
        }
    }
}

#[derive(Clone)]
pub struct Memoized<I: Hash, T: Hash + Staticize + DataType> {
    _input: PhantomData<I>,
    interned: Interned<T>,
}

impl<I: Hash, T: Hash + Staticize + DataType> Memoized<I, T> {
    pub fn interned(&self) -> Interned<T> {
        Interned {
            _value: PhantomData,
            value: self.interned.value,
        }
    }
}

impl<I: Hash, T: Hash + Staticize + DataType<Type = Slice>> Memoized<I, T> {
    pub fn as_slice<'a>(&self) -> &'a [T::SliceValueType] {
        unsafe { self.interned.value.as_slice::<T::SliceValueType>() }
    }
}

impl<I: Hash> Memoized<I, &str> {
    pub fn as_str<'a>(&self) -> &'a str {
        unsafe { self.interned.value.as_str() }
    }
}

impl<I: Hash, T: Hash + Staticize + DataType<Type = Value>> Memoized<I, T> {
    pub fn as_value<'a>(&self) -> &'a T {
        unsafe { self.interned.value.as_value() }
    }
}

impl<I: Hash, T: Hash + Copy + Staticize + DataType> Memoized<I, T>
where
    T::Static: Hash + Copy + Clone + DataType,
{
    pub fn from<G>(input: &I, generator: G) -> Memoized<I, T>
    where
        G: Fn(&I) -> T,
    {
        let mut hasher = DefaultHasher::default();
        input.hash(&mut hasher);
        let input_hash = hasher.finish();
        let type_id = static_type_id::<T>();
        let value_static = MEMOIZED.with(|memoized| {
            match (*memoized)
                .borrow_mut()
                .entry(type_id)
                .or_insert_with(|| HashMap::new())
                .entry(input_hash)
            {
                Entry::Occupied(entry) => *entry.get(),
                Entry::Vacant(entry) => *entry.insert(generator(input).to_static()),
            }
        });
        Memoized {
            _input: PhantomData,
            interned: value_static.into(),
        }
    }
}

impl<I: Hash, T: Hash + Copy + Staticize + DataType<Type = Slice>> Deref for Memoized<I, T> {
    type Target = [T::SliceValueType];

    fn deref(&self) -> &Self::Target {
        match self.interned().value {
            Static::Slice(static_slice) => unsafe { static_slice.as_slice() },
            _ => unreachable!(),
        }
    }
}

impl<I: Hash, T: Hash + PartialEq + Staticize + DataType> PartialEq for Memoized<I, T>
where
    <T as DataType>::SliceValueType: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.interned() == other.interned()
    }
}

impl<I: Hash, T: Hash + Eq + Staticize + DataType> Eq for Memoized<I, T> where
    <T as DataType>::SliceValueType: PartialEq
{
}

impl<I: Hash, T: Hash + PartialOrd + Staticize + DataType> PartialOrd for Memoized<I, T>
where
    <T as DataType>::SliceValueType: PartialEq,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.interned().partial_cmp(&other.interned())
    }
}

impl<I: Hash, T: Hash + Ord + Staticize + DataType> Ord for Memoized<I, T>
where
    <T as DataType>::SliceValueType: PartialEq,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.interned().cmp(&other.interned())
    }
}

impl<I: Hash, T: Hash + Staticize + DataType> Hash for Memoized<I, T>
where
    <T as DataType>::SliceValueType: PartialEq,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.interned().hash(state)
    }
}

impl<I: Hash, T: Hash + Staticize + DataType + std::fmt::Debug> std::fmt::Debug for Memoized<I, T>
where
    <T as DataType>::SliceValueType: PartialEq,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Memoized")
            .field("interned_value", &self.interned())
            .finish()
    }
}

impl<I: Hash, T: Hash + Staticize + DataType + Display> Display for Memoized<I, T>
where
    <T as DataType>::SliceValueType: PartialEq,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.interned().fmt(f)
    }
}

#[test]
fn test_static_alloc() {
    let a = StaticValue::from(37);
    assert_eq!(unsafe { *a.as_value::<i32>() }, 37);
    let b = StaticValue::from(37);
    assert_eq!(a, b); // note: we base equality off of the hash, not the address
    let c = StaticValue::from(8348783947u64);
    assert_ne!(b, c);
    assert_eq!(unsafe { *c.as_value::<u64>() }, 8348783947u64);
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
    let a: Interned<i32> = Interned::from(32);
    let b: Interned<i32> = Interned::from(27);
    assert_ne!(a, b);
    let c: Interned<i32> = Interned::from(32);
    assert_eq!(a, c);
    assert_ne!(b, c);
    assert_eq!(*a.interned_value(), 32);
    assert_eq!(*b.interned_value(), 27);
    assert_eq!(*c.interned_value(), 32);
    assert_eq!(num_interned::<i32>(), initial_interned + 2);
}

#[test]
fn test_interned_showcase() {
    let a: Interned<i32> = 1289.into();
    let b = Interned::from(1289);
    let c: Interned<i32> = 47.into();
    assert_eq!(a, b);
    assert_ne!(a, c);
    assert_eq!(a.as_ptr(), b.as_ptr());
    assert_ne!(b.as_ptr(), c.as_ptr());
    let a: Interned<&str> = "asdf".into();
    assert_ne!(a, "fdsa".into());
    assert_eq!(Interned::from("asdf"), a);
    let a = Interned::from([1, 2, 3, 4, 5].as_slice());
    assert_eq!(a, [1, 2, 3, 4, 5].as_slice().into());
    assert_ne!(a, [4, 1, 7].as_slice().into());
}

#[test]
fn test_interned_into() {
    let a: Interned<i32> = 32.into();
    let b = Interned::from(32);
    assert_eq!(a, b);
    let c: Interned<i32> = 43.into();
    assert_ne!(a, c);
    assert_ne!(c, b);
}

#[test]
fn test_interned_str_types() {
    let a: Interned<&str> = Interned::from("this is a triumph");
    let b: Interned<&str> = Interned::from("I'm making a note here: huge success");
    assert_ne!(a, b);
    assert_ne!(a.interned_str(), b.interned_str());
    assert_eq!(a.interned_str(), "this is a triumph");
    assert_eq!(b.interned_str(), "I'm making a note here: huge success");
    let st = String::from("asdf");
    let c = Interned::from(st.as_str());
    let st2 = String::from("asdf");
    let d = Interned::from(st2.as_str());
    assert_eq!(c, d);
    assert_ne!(c, b);
    let st3 = String::from("nope nope");
    let e = Interned::from(st3.as_str());
    assert_ne!(d, e);
    assert_eq!(c.interned_str().as_ptr(), d.interned_str().as_ptr());
}

#[test]
fn test_interned_deref() {
    let a: Interned<i32> = Interned::from(-99);
    assert_eq!(a.interned_value().abs(), 99);
    let b = Interned::from([5, 6, 7].as_slice());
    assert_eq!(b.len(), 3);
    let c = Interned::from("for the good of all of us except the ones who are dead");
    assert_eq!(c.interned_str().chars().next().unwrap(), 'f');
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
    assert_eq!(a.as_value(), &10);
    assert_ne!(*a.as_value(), 11);
    assert_eq!(*b.interned().interned_value(), 5);
    assert_eq!(*c.as_value(), 10);
    assert_eq!(num_interned::<usize>(), initial_interned + 2);
    assert_eq!(num_memoized::<usize>(), initial_memoized + 2);
}

#[test]
fn test_interned_byte_arrays() {
    let a: Interned<&[u8]> = Interned::from([1u8, 2u8, 3u8].as_slice());
    let b = Interned::from([5u8, 4u8, 3u8, 2u8, 1u8].as_slice());
    assert_ne!(a.interned_slice().as_ptr(), b.interned_slice().as_ptr());
    let c = Interned::from([1u8, 2u8, 3u8].as_slice());
    assert_eq!(a.interned_slice().as_ptr(), c.interned_slice().as_ptr());
    assert_eq!(a.interned_slice(), c.interned_slice());
    assert_eq!(a, c);
    assert_eq!(c, a);
    assert_ne!(a, b);
    assert_ne!(b, a);
}

#[test]
fn test_static_slice() {
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
