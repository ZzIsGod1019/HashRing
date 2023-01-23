use std::{collections::{BTreeMap, hash_map::DefaultHasher}, fmt::Error, hash::{Hash, Hasher}};

use anyhow::{anyhow, Result};

pub struct Ring<'a, T: Eq> {
    link: BTreeMap<i32, Vec<&'a RingItem<T>>>,
}

pub struct RingItem<T: Eq> {
    inner: T,
    tag: String,
}

impl<T: Eq> RingItem<T> {
    pub fn new(t: T, tag: String) -> Self {
        RingItem { inner: t , tag}
    }

    pub fn get_tag(&self) -> String {
        String::from(&self.tag)
    }
}

const MAX_INT : i32 = 65535;
const PER_COUNT : usize = 100;

impl<'a, T: Eq> Ring<'a, T> {
    // 添加项目
    pub fn add_item(&mut self, item: &'a RingItem<T>) -> Result<(), Error> {
        let tag = item.get_tag();
        for i in 0..PER_COUNT {
            let key = self.hash_index( tag.clone()+ &(i.to_string()));

            match self.link.get_mut(&key) {
                Some(items) => items.push(item),
                None => {
                    self.link.insert(key, vec![item]);
                },
            }
        }
        Ok(())
    }

    // 删除项目
    pub fn remove_item(&mut self, item: &'a RingItem<T>) -> Result<(), Error> {
        for i in 0..PER_COUNT {
            let key = self.hash_index(item.get_tag() + &(i.to_string()));

            match self.link.get_mut(&key) {
                Some(items) => {
                    items.retain(|x| x.inner != item.inner);
                },
                None => {},
            }
        }
        Ok(())
    }

    // 选择节点
    pub fn select(&mut self, index: i32) -> Result<&T> {
        let mut res = Err(anyhow!("lock poisoned"));
        for items in self.link.iter_mut() {
            if res.is_err() || *items.0 > index {
                let len = items.1.len() as i32;
                res = Ok(&items.1[(index % len) as usize].inner)
            }
        }
        res
    }

    // 加密后的偏移量
    fn hash_index(&mut self, content: String) -> i32 {
        let mut s = DefaultHasher::new();
        content.hash(&mut s);
        let hash = s.finish();
        (hash % (MAX_INT as u64)) as i32
    }
}

impl<'a, T: Eq> Default for Ring<'a, T> {
    fn default() -> Self {
        Self { link: BTreeMap::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn add_item_test() {
        let mut ring = Ring::default();
        let item1 = RingItem::new(1, String::from("tag:1"));
        ring.add_item(&item1).unwrap();
        let mut node_count = 0;
        for items in ring.link.values() {
            node_count += items.len();
        }
        assert_eq!(node_count, PER_COUNT);
    }

    #[test]
    fn remove_item_test() {
        let mut ring = Ring::default();
        let item1 = RingItem::new(1, String::from("tag:1"));
        let item2 = RingItem::new(2, String::from("tag:2"));
        ring.add_item(&item1).unwrap();
        ring.add_item(&item2).unwrap();
        ring.remove_item(&item2).unwrap();

        let mut node_count = 0;
        for item in ring.link.values() {
            node_count += item.len();
        }
        assert_eq!(node_count, PER_COUNT);

        for items in ring.link {
            for item in items.1 {
                if item.inner == item2.inner {
                    assert_eq!(true, false)
                }
            }
        }
    }

    #[test]
    fn select_test() {
        let mut ring = Ring::default();
        let item1 = RingItem::new(1, String::from("tag:1"));
        let item2 = RingItem::new(2, String::from("tag:2"));
        ring.add_item(&item1).unwrap();
        ring.add_item(&item2).unwrap();

        let hash1 = 4567;
        let t1 = *ring.select(hash1).unwrap();
        let t2 = *ring.select(hash1).unwrap();
        assert_eq!(t1, t2);

        let hash2 = 65535;
        let t3 = *ring.select(hash2).unwrap();
        let t4 = *ring.select(hash2).unwrap();
        assert_eq!(t3, t4)
    }
}