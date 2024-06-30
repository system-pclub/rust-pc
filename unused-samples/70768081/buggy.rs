use std::cmp::Ordering;

struct Node {
    key: i64,
    left_ptr: Option<Box<Node>>,
    right_ptr: Option<Box<Node>>,
}

#[derive(Default)]
pub struct BstSet {
    root: Option<Box<Node>>,
    len: usize,
}
impl BstSet {
    pub fn insert(&mut self, key: i64) -> bool {
        let mut node_ptr = &mut self.root;
        while let Some(node) = node_ptr {
            match node.key.cmp(&key) {
                Ordering::Equal => return false,
                Ordering::Less => {
                    node_ptr = &mut node.right_ptr;
                }
                Ordering::Greater => {
                    node_ptr = &mut node.left_ptr;
                }
            }
        }
        *node_ptr = Some(Box::new(Node {
            key,
            left_ptr: None,
            right_ptr: None,
        }));
        self.len += 1;
        true
    }

    fn find_node_mut(&mut self, key: i64) -> &mut Option<Box<Node>> {
        let mut node_ptr = &mut self.root;
        while let Some(node) = node_ptr {
            match node.key.cmp(&key) {
                Ordering::Equal => break,
                Ordering::Less => {
                    //node_ptr = &mut node_ptr.as_mut().unwrap().right_ptr;
                    node_ptr = &mut node.right_ptr;
                }
                Ordering::Greater => {
                    // node_ptr = &mut node_ptr.as_mut().unwrap().left_ptr;
                    node_ptr = &mut node.left_ptr;
                }
            }
        }
        node_ptr
    }
}

fn task_find_node_mut() -> Vec<Option<i64>> {
    let mut ret = vec![];
    let mut bst = BstSet::default();
    bst.insert(1);
    bst.insert(2);
    bst.insert(3);
    ret.push(bst.find_node_mut(1).as_ref().map(|node| node.key));
    ret.push(bst.find_node_mut(2).as_ref().map(|node| node.key));
    ret.push(bst.find_node_mut(3).as_ref().map(|node| node.key));
    ret.push(bst.find_node_mut(4).as_ref().map(|node| node.key));
    ret
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_insert() {
        let mut bst = BstSet::default();
        assert_eq!(bst.insert(1), true);
        assert_eq!(bst.insert(1), false);
        assert_eq!(bst.insert(2), true);
        assert_eq!(bst.insert(3), true);
        assert_eq!(bst.insert(3), false);
    }

    #[test]
    fn test_find_node_mut() {
        let ret = task_find_node_mut();
        assert_eq!(ret, [Some(1), Some(2), Some(3), None]);
    }
}