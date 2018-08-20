use std::borrow::Borrow;

pub trait KVIterator<K, V> {
    fn valid(&self) -> bool;
    fn key(&self) -> &K;
    fn value(&self) -> &V;
    fn next(&mut self);
    fn advance<Q: ?Sized + Ord>(&mut self, key: &Q)
    where
        K: Borrow<Q>;
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use rand::prelude::random;

    pub fn rand_int_array() -> Vec<u32> {
        let mut v: Vec<u32> = (0..100u32).collect();
        for i in 0..100 {
            let idx = (random::<u32>() % 100) as usize;
            let tmp = v[i];
            v[i] = v[idx];
            v[idx] = tmp;
        }
        v
    }
}
