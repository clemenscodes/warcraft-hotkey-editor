use std::collections::HashMap;

/// Union–find over ability slot strings, used to split the abilities sharing
/// one grid cell into independent collision islands. Two abilities are merged
/// when some unit carries both of them at that cell — the same edge rule the
/// cascade's conflict graph uses. Components that never merge share no carrier
/// unit and therefore never interact.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub(crate) struct SlotIslandPartition {
    parent: HashMap<String, String>,
}

impl SlotIslandPartition {
    pub(crate) fn new() -> Self {
        let parent = HashMap::new();
        Self { parent }
    }

    pub(crate) fn register(&mut self, slot_key: &str) {
        let already_present = self.parent.contains_key(slot_key);
        if already_present {
            return;
        }
        let owned_key = slot_key.to_string();
        let key_copy = owned_key.clone();
        self.parent.insert(key_copy, owned_key);
    }

    pub(crate) fn root(&mut self, slot_key: &str) -> String {
        self.register(slot_key);
        let mut current = slot_key.to_string();
        loop {
            let parent = self
                .parent
                .get(&current)
                .cloned()
                .expect("a registered key always has a parent entry");
            if parent == current {
                return current;
            }
            current = parent;
        }
    }

    pub(crate) fn union(&mut self, left_key: &str, right_key: &str) {
        let left_root = self.root(left_key);
        let right_root = self.root(right_key);
        if left_root != right_root {
            self.parent.insert(left_root, right_root);
        }
    }
}
