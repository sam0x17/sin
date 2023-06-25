extern crate alloc;

use core::{
    hash::{Hash, Hasher},
    marker::PhantomData,
    ops::Deref,
};

use ahash::AHasher;
use alloc::{boxed::Box, sync::Arc};
use core::any::{Any, TypeId};
use dashmap::DashMap;
use once_cell::sync::Lazy;
use parking_lot::RwLock;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct RawHeapPtr(*const ());

unsafe impl Send for RawHeapPtr {}
unsafe impl Sync for RawHeapPtr {}

impl RawHeapPtr {
    pub const fn as_ptr<T>(&self) -> *const T {
        self.0 as *const T
    }

    pub const fn as_ref<'a, T>(&self) -> &'a T {
        unsafe { &*(self.0 as *const T) }
    }

    pub fn from<T>(value: T) -> Self {
        RawHeapPtr((Box::leak(Box::from(value)) as *const T) as *const ())
    }
}

/// `hash(I) => ptr`
static INTERNED_CACHE: Lazy<DashMap<u64, RawHeapPtr>> = Lazy::new(|| DashMap::new());

/// `hash(T) => ptr`
static INTERNED: Lazy<DashMap<u64, RawHeapPtr>> = Lazy::new(|| DashMap::new());

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct InternedCache<T: Hash, I: Hash> {
    ptr: RawHeapPtr,
    _value: PhantomData<T>,
    _input: PhantomData<I>,
}

impl<T: Hash, I: Hash> InternedCache<T, I> {
    pub fn from_input<R: AsRef<I>, G>(input: R, generator: G) -> Self
    where
        G: Fn() -> T,
    {
        let input = input.as_ref();
        let mut hasher = AHasher::default();
        input.hash(&mut hasher);
        let input_hash = hasher.finish();
        let generate = || -> RawHeapPtr { RawHeapPtr::from(generator()) };
        let entry = INTERNED_CACHE.entry(input_hash).or_insert_with(generate);
        let mut hasher = AHasher::default();
        entry.as_ref::<T>().hash(&mut hasher);
        let value_hash = hasher.finish();
        let ptr = *INTERNED.entry(value_hash).or_insert(*entry);
        InternedCache {
            ptr,
            _value: PhantomData,
            _input: PhantomData,
        }
    }

    pub fn from_value<R: AsRef<T>>(value: R) -> Self {
        let value = value.as_ref();
        let mut hasher = AHasher::default();
        value.hash(&mut hasher);
        let value_hash = hasher.finish();
        let ptr = *INTERNED
            .entry(value_hash)
            .or_insert(RawHeapPtr::from(value));
        InternedCache {
            ptr,
            _value: PhantomData,
            _input: PhantomData,
        }
    }
}

// pub trait Interned<T: Hash> {
//     pub fn from() -> &'static Lazy<RwLock<T>> {
//         let type_id = TypeId::of::<T>();
//         static LOCK: Lazy<RwLock<Self::K>> = Lazy::new(|| RwLock::new(false));
//         &LOCK
//     }
// }

// static INTERNED_BYTES: Lazy<RwLock<HashMap<u64, &'static [u8]>>> =
//     Lazy::new(|| RwLock::new(HashMap::new()));

// #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
// pub struct InternedBytes {
//     slice: &'static [u8],
// }

// impl InternedBytes {
//     pub fn num_interned() -> usize {
//         INTERNED_BYTES.read().len()
//     }
// }

// impl<const N: usize> From<&[u8; N]> for InternedBytes {
//     fn from(value: &[u8; N]) -> Self {
//         InternedBytes::from(value as &[u8])
//     }
// }

// impl From<&[u8]> for InternedBytes {
//     fn from(value: &[u8]) -> Self {
//         let mut hasher = AHasher::default();
//         value.hash(&mut hasher);
//         let hash = hasher.finish();

//         // read section
//         let data = INTERNED_BYTES.read();
//         if let Some(slice) = data.get(&hash) {
//             return InternedBytes { slice: *slice };
//         }
//         drop(data);

//         // write section (if applicable)
//         let mut data = INTERNED_BYTES.write();

//         // just in case some other writer has come in since our read lock expired
//         if let Some(slice) = data.get(&hash) {
//             return InternedBytes { slice };
//         }

//         // intern new bytes in the hash map
//         let ptr = Box::leak(Box::from(value));
//         let slice = ptr as &'static [u8];
//         data.insert(hash, slice);
//         return InternedBytes { slice };
//     }
// }

// impl AsRef<[u8]> for InternedBytes {
//     fn as_ref(&self) -> &[u8] {
//         self.slice
//     }
// }

// #[test]
// fn test_interned_bytes() {
//     let initial = InternedBytes::num_interned();
//     assert_eq!(InternedBytes::num_interned(), initial);
//     assert_eq!(
//         InternedBytes::from(&[1, 2, 3, 4, 5]).as_ref(),
//         &[1, 2, 3, 4, 5]
//     );
//     assert_eq!(InternedBytes::num_interned(), initial + 1);
//     assert_eq!(InternedBytes::from(&[3, 2]).as_ref(), &[3, 2]);
//     assert_eq!(InternedBytes::num_interned(), initial + 2);
//     let var1 = InternedBytes::from(&[1, 2, 3]);
//     let var2 = InternedBytes::from(&[]);
//     let var3 = InternedBytes::from(&[99, 100, 101]);
//     assert_eq!(InternedBytes::num_interned(), initial + 5);
//     let var4 = InternedBytes::from(&[1, 2, 3]);
//     assert_eq!(InternedBytes::num_interned(), initial + 5);
//     assert_eq!(var1.as_ref().as_ptr(), var4.as_ref().as_ptr());
//     assert_ne!(var2.as_ref().as_ptr(), var3.as_ref().as_ptr());
// }

// #[test]
// fn test_interned_bytes_traits() {
//     use crate::util::*;
//     assert_send::<InternedBytes>();
//     assert_sync::<InternedBytes>();
// }
