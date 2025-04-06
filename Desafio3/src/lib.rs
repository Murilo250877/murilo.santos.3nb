// Estrutura do nó da árvore
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

// Estrutura da BST
pub struct BST {
    root: Option<Box<Node>>,
}

impl BST {
    pub fn new() -> Self {
        BST { root: None }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn insert(&mut self, value: i32) {
        self.root = Self::insert_node(self.root.take(), value);
    }

    fn insert_node(node: Option<Box<Node>>, value: i32) -> Option<Box<Node>> {
        match node {
            Some(mut current) => {
                if value < current.value {
                    current.left = Self::insert_node(current.left.take(), value);
                } else if value > current.value {
                    current.right = Self::insert_node(current.right.take(), value);
                }
                Some(current)
            }
            None => Some(Box::new(Node {
                value,
                left: None,
                right: None,
            })),
        }
    }

    pub fn search(&self, value: i32) -> bool {
        Self::search_node(&self.root, value)
    }

    fn search_node(node: &Option<Box<Node>>, value: i32) -> bool {
        match node {
            Some(current) => {
                if value == current.value {
                    true
                } else if value < current.value {
                    Self::search_node(&current.left, value)
                } else {
                    Self::search_node(&current.right, value)
                }
            }
            None => false,
        }
    }
}

// Testes
#[cfg(test)]
mod bst_tests {
    use super::*;

    #[test]
    fn test_bst_new_and_empty() {
        let bst = BST::new();
        assert!(bst.is_empty());
    }

    #[test]
    fn test_bst_insert_and_search() {
        let mut bst = BST::new();

        bst.insert(10);
        bst.insert(5);
        bst.insert(15);

        assert!(bst.search(10));
        assert!(bst.search(5));
        assert!(bst.search(15));
        assert!(!bst.search(20));
        assert!(!bst.is_empty());
    }
}
