use rand::prelude::{SliceRandom, ThreadRng};
use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::Debug,
    hash::Hash,
    rc::{Rc, Weak},
    time::{Duration, Instant},
};

/// - parent: Node
/// - childen: HashMap<Action, Node>
/// - visit: u64,
/// - score: i64,
pub struct Node<A> {
    parent: Weak<Node<A>>,
    children: RefCell<HashMap<A, Rc<Node<A>>>>,
    visit: RefCell<u64>,
    score: RefCell<i64>,
}

impl<A> Default for Node<A> {
    fn default() -> Self {
        Self {
            parent: Default::default(),
            children: Default::default(),
            visit: Default::default(),
            score: Default::default(),
        }
    }
}

impl<A: Debug + Copy> Debug for Node<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn formatter<A: Debug + Copy>(
            node: &Node<A>,
            f: &mut std::fmt::Formatter<'_>,
            depth: usize,
        ) -> std::fmt::Result {
            for (action, child) in node.children.borrow().iter() {
                let space = (0..depth).map(|_| '-').collect::<String>();
                writeln!(
                    f,
                    "{space}{action:?}(score: {}/{} - {:.2})",
                    child.score.borrow(),
                    child.visit.borrow(),
                    child.eval(),
                )?;
                formatter(child, f, depth + 2)?;
            }
            Ok(())
        }
        writeln!(f, "{}/{}", self.score.borrow(), self.visit.borrow())?;

        formatter(self, f, 0)
    }
}

impl<A: Copy> Node<A> {
    pub fn eval(&self) -> f64 {
        let score = *self.score.borrow() as f64;
        let visit = *self.visit.borrow() as f64;
        let pvisit = *self.parent.upgrade().unwrap().visit.borrow();
        score / visit + (2. * (pvisit as f64).ln() / visit).sqrt()
    }
    pub fn best_action(&self) -> Option<A> {
        self.children
            .borrow()
            .iter()
            .max_by(|a, b| a.1.eval().partial_cmp(&b.1.eval()).unwrap())
            .map(|x| *x.0)
    }

    pub fn worst_action(&self) -> Option<A> {
        self.children
            .borrow()
            .iter()
            .min_by(|a, b| a.1.eval().partial_cmp(&b.1.eval()).unwrap())
            .map(|x| *x.0)
    }
}

pub struct Garbage<A>(Vec<Rc<Node<A>>>);

impl<A> std::ops::Deref for Garbage<A> {
    type Target = Vec<Rc<Node<A>>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<A> std::ops::DerefMut for Garbage<A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// MCTS stands for Monte Carlo Tree Search
/// - A: Action
pub trait Mcts<A: Debug + Copy + Eq + Hash>: Debug + Clone {
    /// Generate all possible actions
    fn fill(&self, actions: &mut Vec<A>);

    /// Update the system
    fn update(&mut self, action: A) -> i64;

    fn clear_and_fill(&self, actions: &mut Vec<A>) {
        actions.clear();
        self.fill(actions)
    }

    fn rollout(&mut self, rng: &mut ThreadRng, node: &Rc<Node<A>>, actions: &mut Vec<A>) {
        self.clear_and_fill(actions);
        *node.visit.borrow_mut() += 1;

        if let Some(&action) = actions.choose(rng) {
            // Update game with action
            let score = self.update(action);

            let mut node_children = node.children.borrow_mut();
            if let Some(child) = node_children.get_mut(&action) {
                // Update child node
                let past_score = *child.score.borrow();
                *child.score.borrow_mut() += score;

                self.rollout(rng, child, actions);

                // Update node (backpropagation)
                *node.score.borrow_mut() += *child.score.borrow() - past_score;
            } else {
                // Create child node
                let child = Rc::new(Node {
                    parent: Rc::downgrade(node),
                    children: Default::default(),
                    visit: RefCell::new(0),
                    score: RefCell::new(score),
                });

                self.rollout(rng, &child, actions);

                // Update node (backpropagation)
                *node.score.borrow_mut() += *child.score.borrow();
                node_children.insert(action, child);
            }
        }
    }

    fn mcts(
        &mut self,
        rng: &mut ThreadRng,
        total_time: Duration,
        node: &Rc<Node<A>>,
        actions: &mut Vec<A>,
    ) {
        let now = Instant::now();
        let mut max_time = Duration::default();

        // While there is time rollout on each child
        while now.elapsed() + max_time < total_time {
            let t = Instant::now();
            self.clone().rollout(rng, node, actions);
            max_time = std::cmp::max(max_time, t.elapsed())
        }
    }

    fn advance(&mut self, garbage: &mut Garbage<A>, node: Rc<Node<A>>, action: A) -> Rc<Node<A>> {
        self.update(action);
        let child = node
            .children
            .borrow_mut()
            .remove(&action)
            .unwrap_or_default();
        garbage.push(node);
        child
    }
}

#[test]
fn test() {
    use super::Mcts;

    #[derive(Debug, Clone)]
    enum Game {
        State0,
        State1,
        State2,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum Action {
        A0,
        A1,
        A2,
        A3,
    }

    impl Mcts<Action> for Game {
        fn fill(&self, actions: &mut Vec<Action>) {
            match self {
                Game::State0 => {
                    actions.push(Action::A0);
                    actions.push(Action::A1);
                }
                Game::State1 => {
                    actions.push(Action::A2);
                    actions.push(Action::A3);
                }
                Game::State2 => (),
            }
        }

        fn update(&mut self, action: Action) -> i64 {
            match action {
                Action::A0 => 0,
                Action::A1 => {
                    *self = Game::State1;
                    0
                }
                Action::A2 => {
                    *self = Game::State2;
                    0
                }
                Action::A3 => {
                    *self = Game::State2;
                    1
                }
            }
        }
    }

    let mut game = Game::State0;
    let mut rng = rand::thread_rng();
    let node = Rc::new(Node::default());
    let mut actions = Vec::new();

    game.mcts(&mut rng, Duration::from_micros(5000), &node, &mut actions);

    println!("{node:?}")
}
