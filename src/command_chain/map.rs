use im::{OrdMap, OrdSet, Vector};

#[derive(Clone)]
pub enum Map<V: Clone> {
    Empty,
    NonEmpty(Node<V>),
}

#[derive(Clone)]
pub enum Node<V: Clone> {
    SubTree(OrdMap<char, Node<V>>),
    Value(V),
}

pub enum SearchResult<V: Clone> {
    Value(V),
    Children(OrdSet<char>),
    None,
}

impl<V: Clone> Map<V> {
    pub fn new() -> Self {
        Map::Empty
    }

    pub fn with(self: &Self, key: String, value: V) -> Result<Self, &'static str> {
        let chars = key_to_vec(key);
        let node = match self {
            Map::Empty => Node::rec_add(chars, value),
            Map::NonEmpty(node) => node.rec_set(chars, value),
        }?;
        Ok(Map::NonEmpty(node))
    }

    pub fn without(self: &Self, key: String) -> Self {
        let chars = key_to_vec(key);
        match self {
            Map::Empty => Map::Empty,
            Map::NonEmpty(node) => match node.rec_remove(chars) {
                None => Map::Empty,
                Some(node) => Map::NonEmpty(node),
            },
        }
    }

    pub fn search(self: &Self, key: String) -> SearchResult<V> {
        let chars = key_to_vec(key);
        match self {
            Map::Empty => SearchResult::None,
            Map::NonEmpty(node) => node.rec_search(chars),
        }
    }
}

impl<V: Clone> Node<V> {
    fn rec_set(self: &Self, mut key: Vector<char>, value: V) -> Result<Self, &'static str> {
        match (key.pop_front(), self) {
            (None, Node::Value(_)) => Ok(Node::Value(value)),
            (None, Node::SubTree(_)) => Err(
                "Attempting to add a prefix of a command chain which already exists in the mapping",
            ),
            (Some(c), Node::SubTree(ord_map)) => {
                let updated_node = match ord_map.get(&c) {
                    None => Node::rec_add(key, value)?,
                    Some(node) => node.rec_set(key, value)?,
                };
                Ok(Node::SubTree(ord_map.update(c, updated_node)))
            }
            (Some(_), Node::Value(_)) => {
                Err("Attempting to add a command chain whose prefix already exists in the mapping")
            }
        }
    }

    fn rec_add(mut key: Vector<char>, value: V) -> Result<Self, &'static str> {
        match key.pop_front() {
            None => Ok(Node::Value(value)),
            Some(c) => {
                let sub_node = Node::rec_add(key, value)?;
                Ok(Node::SubTree(OrdMap::unit(c, sub_node)))
            }
        }
    }

    fn rec_remove(self: &Self, mut key: Vector<char>) -> Option<Self> {
        match (key.pop_front(), self) {
            (None, Node::Value(_)) => None,
            (None, Node::SubTree(_)) | (Some(_), Node::Value(_)) => Some(self.clone()),
            (Some(c), Node::SubTree(ord_map)) => match ord_map.get(&c) {
                None => Some(self.clone()),
                Some(node) => match node.rec_remove(key) {
                    Some(node) => Some(Node::SubTree(ord_map.update(c, node))),
                    None => {
                        let ord_map = ord_map.without(&c);
                        match ord_map.is_empty() {
                            false => Some(Node::SubTree(ord_map)),
                            true => None,
                        }
                    }
                },
            },
        }
    }

    fn rec_search(self: &Self, mut key: Vector<char>) -> SearchResult<V> {
        match (key.pop_front(), self) {
            (None, Node::Value(value)) => SearchResult::Value(value.clone()),
            (None, Node::SubTree(ord_tree)) => {
                SearchResult::Children(ord_tree.keys().fold(OrdSet::new(), |mut ord_set, key| {
                    ord_set.insert(*key);
                    ord_set
                }))
            }
            (Some(_), Node::Value(_)) => SearchResult::None,
            (Some(c), Node::SubTree(ord_tree)) => match ord_tree.get(&c) {
                None => SearchResult::None,
                Some(node) => node.rec_search(key),
            },
        }
    }
}

fn key_to_vec(key: String) -> Vector<char> {
    let mut chars = Vector::new();
    key.chars().into_iter().for_each(|c| {
        chars.push_back(c);
    });
    chars
}
