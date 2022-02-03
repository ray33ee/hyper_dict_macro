
///Demonic conversion to convert a slice into a u64, padding with zeros if needed
/*fn str_to_u64(s: &str) -> u64 {

    let s = s.as_bytes();

    //Take an 8 byte slice of s, even if this means indexing past the end of s
    let past_the_end = unsafe { std::slice::from_raw_parts((&s[index]) as *const u8, 8) };

    //Converting &[u8] to u64 this way is fast, but if this works as be on one platform and le on another, this may break the map hash function so we always explicitly give little endian.
    //
    //let past_the_end_value = past_the_end.as_ptr() as * const u64;
    let past_the_end_value = u64::from_le_bytes(std::convert::TryInto::try_into(past_the_end).unwrap());

    if index + 8 > s.len() {
        let pad = 8 - s.len() + index;

        //Shift left then right to pad the top with zeros
        let trimmed = past_the_end_value << (pad * 8);
        trimmed >> (pad * 8)
    } else {
        past_the_end_value
    }

}*/

///32-bit hash function taken from https://stackoverflow.com/questions/664014/what-integer-hash-function-are-good-that-accepts-an-integer-hash-key.
///Might need changing in the future
/*fn hash(value: u64) -> u64 {
    let mut x = std::num::Wrapping(value ^ k);
    x = (x ^ (x >> 31) ^ (x >> 62)) * std::num::Wrapping(0x319642b2d24d8ec3);
    x = x ^ (x >> 30) ^ (x >> 60);
    x.0
}*/

pub trait HyperDict<K, V> {
    ///Get the size of the dictionary (number of value or key-value pairs)
    fn len(&self) -> usize;

    ///Get a mutable iterator over the values
    fn iter_mut(&self) -> std::slice::IterMut<V>;
}

pub trait AccessKeyValue<K, V> {
    ///Get the value from the specified key
    fn get(&self, key: K) -> Option<&V>;

    ///Get an iterator over the key-value pairs
    fn iter(&self) -> std::slice::Iter<(K, V)>;
}

pub trait AccessDictValueOnly<K, V> {
    ///Get the value from the specified key
    fn get(&self, key: K) -> &V;

    ///Get an iterator over the values
    fn iter(&self) -> std::slice::Iter<V>;
}