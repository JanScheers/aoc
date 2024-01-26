struct Solution;
use aoc::TreeNode;
use std::cell::RefCell;
use std::iter::zip;
use std::rc::Rc;

impl Solution {
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn is_leaf(node: &Rc<RefCell<TreeNode>>) -> bool {
            let node = node.borrow();
            node.left.is_none() && node.right.is_none()
        }
        let leaves1: Vec<_> = root1.unwrap().borrow().in_iter().filter(is_leaf).collect();
        let leaves2: Vec<_> = root2.unwrap().borrow().in_iter().filter(is_leaf).collect();
        leaves1.len() == leaves2.len()
            && zip(leaves1, leaves2).all(|(node1, node2)| node1.borrow().val == node2.borrow().val)
    }
}

/**
 * Your RandomizedCollection object will be instantiated and called as such:
 * let obj = RandomizedCollection::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */
fn main() {
    let tree = TreeNode::deserialize("[3,5,1,6,2,9,8,null,null,7,4]");
    let tree2 = TreeNode::deserialize("[3,5,1,6,7,4,2,null,null,null,null,null,null,9,8]");
    println!("{}", Solution::leaf_similar(tree, tree2));
}
