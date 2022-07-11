use rand::prelude::{SliceRandom, ThreadRng};
use std::ops::Add;

use crate::Obj;

pub trait MonteCarlo<A: Copy + std::fmt::Debug, T: Add<Output = T> + PartialOrd + std::fmt::Debug>:
    Clone
{
    /// Generate all possible actions
    fn fill(&self, actions: &mut Vec<A>);
    /// Update the system
    fn update(&mut self, action: A) -> T;

    fn rng() -> ThreadRng {
        rand::thread_rng()
    }

    fn run(&self, action: A, actions: &mut Vec<A>, rng: &mut ThreadRng) -> T {
        let mut run = self.clone();
        let mut score = run.update(action);
        run.fill(actions);

        while let Some(action) = actions.choose(rng) {
            score = score + run.update(*action);
            run.fill(actions);
        }

        score
    }

    fn multi_run(&self, action: A, n: usize, actions: &mut Vec<A>, rng: &mut ThreadRng) -> T {
        (0..n)
            .map(|_| self.run(action, actions, rng))
            .reduce(|acc, score| acc + score)
            .unwrap()
    }

    fn mc_play(&self, n: usize, actions: &mut Vec<A>, rng: &mut ThreadRng, obj: Obj) -> Option<A> {
        assert!(n > 0);
        self.fill(actions);
        let mut _actions = vec![];

        match obj {
            Obj::Max => actions
                .iter()
                .copied()
                .map(|action| (self.multi_run(action, n, &mut _actions, rng), action))
                .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
                .map(|max| max.1),
            Obj::Min => actions
                .iter()
                .copied()
                .map(|action| (self.multi_run(action, n, &mut _actions, rng), action))
                .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
                .map(|max| max.1),
        }
    }
}
