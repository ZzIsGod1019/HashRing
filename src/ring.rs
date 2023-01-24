use std::{collections::{BTreeMap, hash_map::DefaultHasher, HashMap}, hash::{Hash, Hasher}};

use anyhow::{anyhow, Result};

pub struct Ring<T> {
    link: BTreeMap<i32, Vec<String>>,
    map: HashMap<String, T>
}

const MAX_INT : i32 = 65535;
const PER_COUNT : usize = 100;

impl<T> Ring<T> {
    // 添加项目
    pub fn add_item(&mut self, item: T, tag: String) -> Result<()> {
        if self.map.contains_key(&tag) {
            return Err(anyhow!("tag exists"));
        }
        self.map.insert(tag.clone(), item);

        for i in 0..PER_COUNT {
            let key = self.hash_index( tag.clone() + &(i.to_string()));

            match self.link.get_mut(&key) {
                Some(items) => items.push(tag.clone()),
                None => {
                    self.link.insert(key, vec![tag.clone()]);
                },
            }
        }
        Ok(())
    }

    // 删除项目
    pub fn remove_item(&mut self, tag: String) -> Result<()> {
        // let item = self.map.get(&tag).unwrap();
        for i in 0..PER_COUNT {
            let key = self.hash_index(tag.clone() + &(i.to_string()));

            match self.link.get_mut(&key) {
                Some(items) => {
                    items.retain(|x| x != &tag);
                },
                None => {},
            }
        }
        Ok(())
    }

    // 选择节点
    pub fn select(&mut self, index: i32) -> Option<&T> {
        let mut tag = None;
        for items in self.link.iter_mut() {
            if tag.is_none() || *items.0 > index {
                let len = items.1.len() as i32;
                tag = Some(items.1[(index % len) as usize].clone());
            }
        }
        if tag.is_some() {
            return self.map.get(&(tag.unwrap()));
        }
        None
    }

    // 加密后的偏移量
    fn hash_index(&mut self, tag: String) -> i32 {
        let mut s = DefaultHasher::new();
        tag.hash(&mut s);
        let hash = s.finish();
        (hash % (MAX_INT as u64)) as i32
    }
}

impl<T> Default for Ring<T> {
    fn default() -> Self {
        Self {
            link: BTreeMap::new(),
            map: HashMap::new()
        }
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
        ring.add_item(1, String::from("tag:1")).unwrap();
        let mut node_count = 0;
        for items in ring.link.values() {
            node_count += items.len();
        }
        assert_eq!(node_count, PER_COUNT);
    }

    #[test]
    fn remove_item_test() {
        let mut ring = Ring::default();
        ring.add_item(1, String::from("tag:1")).unwrap();
        ring.add_item(2, String::from("tag:2")).unwrap();
        ring.remove_item(String::from("tag:2")).unwrap();

        let mut node_count = 0;
        for item in ring.link.values() {
            node_count += item.len();
        }
        assert_eq!(node_count, PER_COUNT);

        for items in ring.link {
            for item in items.1 {
                if item == String::from("tag:2") {
                    assert_eq!(true, false);
                }
            }
        }
    }

    #[test]
    fn select_test() {
        let mut ring = Ring::default();
        ring.add_item(1, String::from("tag:1")).unwrap();
        ring.add_item(2, String::from("tag:2")).unwrap();

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