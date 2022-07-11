use core::cmp::max;

#[derive(Debug, Default)]
pub struct Tree<T> {
    pub node: Node<T>,
    pub depth: usize,
}

#[derive(Debug, Default)]
pub struct Node<T> {
    pub value: T,
    pub childs: Vec<Node<T>>,
}

impl<T> Tree<T> {
    pub fn dfs(&mut self) {
        let mut stack = vec![(0, &mut self.node)];

        while let Some((depth, node)) = stack.pop() {
            // Do something here
            self.depth = max(self.depth, depth);
            for child in &mut node.childs {
                stack.push((depth + 1, child));
            }
        }
    }
}
