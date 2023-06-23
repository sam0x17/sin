extern crate alloc;

use core::hash::{Hash, Hasher};

use ahash::AHasher;
use alloc::boxed::Box;
use hashbrown::HashMap;
use once_cell::sync::Lazy;
use spin::Mutex;

const INTERNED_BYTES: Lazy<Mutex<HashMap<u64, Box<[u8]>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct InternedBytes {
    slice: &'static [u8],
}

impl InternedBytes {
    pub fn num_interned() -> usize {
        let binding = INTERNED_BYTES;
        let len = binding.lock().len();
        len
    }
}

impl<const N: usize> From<&[u8; N]> for InternedBytes {
    fn from(value: &[u8; N]) -> Self {
        InternedBytes::from(value as &[u8])
    }
}

impl From<&[u8]> for InternedBytes {
    fn from(value: &[u8]) -> Self {
        let mut hasher = AHasher::default();
        value.hash(&mut hasher);
        let hash = hasher.finish();

        let binding = INTERNED_BYTES;
        let mut data = binding.lock();
        let entry = data.entry(hash).or_insert(Box::from(value));
        let ptr = entry.as_ptr();
        let len = entry.len();
        let slice = unsafe { core::slice::from_raw_parts(ptr, len) };
        InternedBytes { slice }
    }
}

impl AsRef<[u8]> for InternedBytes {
    fn as_ref(&self) -> &[u8] {
        self.slice
    }
}

#[test]
fn test_interned_bytes() {
    let initial = InternedBytes::num_interned();
    assert_eq!(InternedBytes::num_interned(), initial);
    assert_eq!(
        InternedBytes::from(&[1, 2, 3, 4, 5]).as_ref(),
        &[1, 2, 3, 4, 5]
    );
    assert_eq!(InternedBytes::num_interned(), initial + 1);
    assert_eq!(InternedBytes::from(&[3, 2]).as_ref(), &[3, 2]);
    assert_eq!(InternedBytes::num_interned(), initial + 2);
    let var1 = InternedBytes::from(&[1, 2, 3]);
    let var2 = InternedBytes::from(&[]);
    let var3 = InternedBytes::from(&[99, 100, 101]);
    assert_eq!(InternedBytes::num_interned(), initial + 5);
    let var4 = InternedBytes::from(&[1, 2, 3]);
    assert_eq!(InternedBytes::num_interned(), initial + 5);
    assert_eq!(var1.as_ref().as_ptr(), var4.as_ref().as_ptr());
    assert_ne!(var2.as_ref().as_ptr(), var3.as_ref().as_ptr());
}

#[test]
fn test_interned_bytes_traits() {
    use crate::util::*;
    assert_send::<InternedBytes>();
    assert_sync::<InternedBytes>();
}
