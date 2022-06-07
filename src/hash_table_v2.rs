pub trait Hash: Eq {
    fn hash(&self) -> i64;
}

// Use linked list to handle the hash conflicts for the same bucket.
pub struct HashMap<K: Hash, V> {
    size: usize,
    buckets: Vec<Ptr<K, V>>,
}

type Ptr<K, V> = Option<Box<Node<K, V>>>;
pub struct Node<K: Hash, V> {
    key: K,
    val: V,
    next: Ptr<K, V>,
}

pub struct Iter<'a, K: Hash, V> {
    index: usize,
    cur: &'a Ptr<K, V>,
}

fn hash<K: Hash>(bucket_num: usize, key: &K) -> usize {
    let m = bucket_num as i64;
    ((key.hash() % m + m) % m) as usize
}

impl<K, V> HashMap<K, V>
where
    K: Hash,
{
    pub fn new() -> Self {
        HashMap {
            size: 0,
            buckets: (0..4).map(|_| None).collect(),
        }
    }

    pub fn bucket_num(&self) -> usize {
        return self.buckets.capacity();
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut Node<K, V>> {
        let h = hash(self.bucket_num(), key);
        let mut ptr = &mut self.buckets[h];
        while let Some(node) = ptr.as_deref_mut() {
            if node.key == *key {
                return Some(node);
            }
            ptr = &mut node.next;
        }
        return None;
    }

    fn rehash(&mut self) {
        // let mut buckets: Vec<Ptr<K, V>> = (0..self.bucket_num()*2).map(|_| None).collect();
        // for h in 0..self.bucket_num() {
        //     let mut ptr = &mut self.buckets[h];
        //     while let Some(node) = ptr.as_deref_mut() {
        //         let h = hash(buckets.capacity(), &node.key);
        //         ptr = &mut node.next.take();

        //         node.next = buckets[h].take();
        //         buckets[h] = Some(Box::new(*node));
        //     }
        // }
    }

    pub fn put(&mut self, key: K, val: V) {
        let h = hash(self.bucket_num(), &key);

        match self.get_mut(&key) {
            None => {
                self.buckets[h] = Some(Box::new(Node {
                    key,
                    val,
                    next: self.buckets[h].take(),
                }));
                self.size += 1;
            }
            Some(node) => {
                *node = Node {
                    key,
                    val,
                    next: node.next.take(),
                }
            }
        }

        if self.size >= self.bucket_num() * 2 {
            self.rehash();
        }
    }

    pub fn get(&self, key: K) -> Option<&V> {
        todo!()
    }

    pub fn remove(&mut self, key: K) -> Option<V> {
        todo!()
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, K, V> {
        Iter {
            index: 0,
            cur: &None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn basics() {}
}
