use std::slice::{IterMut, Iter};

pub struct OperatorHD(Vec<(& 'static str, f32)>);

impl OperatorHD {

    pub fn new() -> Self {
        //NOTE: how these are in a different order than specified, this will need to be computed by the macro
        Self(vec![("sub", -1.0), ("sqrt", 0.5), ("add", 1.0), ("div" ,3.0), ("mul", 2.0)])
    }

    ///Demonic conversion to convert a slice into a u64, padding with zeros if needed
    fn str_to_u64(s: &str) -> u64 {

        let s = s.as_bytes();

        //Take an 8 byte slice of s, even if this means indexing past the end of s
        let past_the_end = unsafe { std::slice::from_raw_parts((&s[0]) as *const u8, 8) };

        //Converting &[u8] to u64 this way is fast, but if this works as be on one platform and le on another, this may break the map hash function so we always explicitly give little endian.
        //
        //let past_the_end_value = past_the_end.as_ptr() as * const u64;
        let past_the_end_value = u64::from_le_bytes(std::convert::TryInto::try_into(past_the_end).unwrap());

        if 0 + 8 > s.len() {
            let pad = 8 - s.len() + 0;

            //Shift left then right to pad the top with zeros
            let trimmed = past_the_end_value << (pad * 8);
            trimmed >> (pad * 8)
        } else {
            past_the_end_value
        }

    }

    ///32-bit hash function taken from https://stackoverflow.com/questions/664014/what-integer-hash-function-are-good-that-accepts-an-integer-hash-key.
    ///Might need changing in the future
    fn hash(value: u64) -> u64 {
        let mut x = std::num::Wrapping(value ^ 0x35);
        x = (x ^ (x >> 31) ^ (x >> 62)) * std::num::Wrapping(0x319642b2d24d8ec3);
        x = x ^ (x >> 30) ^ (x >> 60);
        x.0
    }
}

impl crate::traits::HyperDict<& 'static str, f32> for OperatorHD {
    fn len(&self) -> usize { 5 }

    fn iter_mut(&self) -> IterMut<f32> {
        unimplemented!()
    }
}

impl crate::traits::AccessKeyValue<& 'static str, f32> for OperatorHD {
    fn get(&self, key: &'static str) -> Option<&f32> {
        let (k, v) = &self.0[(Self::hash(Self::str_to_u64(key)) % 5) as usize];
        if *k == key {
            Some(v)
        } else {
            None
        }
    }

    fn iter(&self) -> Iter<(&'static str, f32)> {
        self.0.iter()
    }
}

pub struct ArbitraryHD(Vec<i32>);

impl ArbitraryHD {

    pub fn new() -> Self {
        //NOTE: how these are in a different order than specified, this will need to be computed by the macro
        Self(vec![6, 1, 4, 3, 5, 2])
    }
    /*       [0, 23, 67, 32, 76, 8]
    Starting search with 11 threads...
    Found constant: 0x65 (elapsed: 532.6µs)
    Indices: [1, 5,  3,  2,  4,  0]*/

    //{0: 1, 23: 2, 67: 3, 32: 4, 76: 5, 8: 6}

    ///32-bit hash function taken from https://stackoverflow.com/questions/664014/what-integer-hash-function-are-good-that-accepts-an-integer-hash-key.
    ///Might need changing in the future
    fn hash(value: u64) -> u64 {
        let mut x = std::num::Wrapping(value ^ 0x65);
        x = (x ^ (x >> 31) ^ (x >> 62)) * std::num::Wrapping(0x319642b2d24d8ec3);
        x = x ^ (x >> 30) ^ (x >> 60);
        x.0
    }
}

impl crate::traits::HyperDict<i32, i32> for ArbitraryHD {
    fn len(&self) -> usize { 6 }

    fn iter_mut(&self) -> IterMut<i32> {
        unimplemented!()
    }
}

impl crate::traits::AccessDictValueOnly<i32, i32> for ArbitraryHD {
    fn get(&self, key: i32) -> &i32 {
        &self.0[(Self::hash(key as u64) % 6) as usize]
    }

    fn iter(&self) -> Iter<i32> {
        self.0.iter()
    }
}
