use rand::prelude::{SliceRandom, ThreadRng};
use std::time::{Duration, Instant};

/// A: Action
///
/// The more depth, the less precision
pub trait MonteCarlo<A: Copy + std::fmt::Debug>: Clone {
    /// Generate all possible actions, don't forget to clear
    fn fill(&self, actions: &mut Vec<A>);
    /// Update the system
    fn update(&mut self, action: A) -> f32;

    fn run(&self, action: A, actions: &mut Vec<A>, max_depth: usize, rng: &mut ThreadRng) -> f32 {
        let mut run = self.clone();
        let mut score = run.update(action);
        let mut depth = 1;
        run.fill(actions);

        while let Some(action) = actions.choose(rng) {
            if depth == max_depth {
                return score;
            }
            score += (0.7f32).powi(depth as i32) * run.update(*action);
            depth += 1;
            run.fill(actions);
        }

        score
    }

    fn multi_run(
        &self,
        max: Duration,
        action: A,
        actions: &mut Vec<A>,
        max_depth: usize,
        rng: &mut ThreadRng,
    ) -> f32 {
        let now = Instant::now();
        let mut max_time = Duration::default();
        let mut s = 0.;
        let mut n = 0.;

        while now.elapsed() + max_time < max {
            let t = Instant::now();
            s += self.run(action, actions, max_depth, rng);
            n += 1.;
            max_time = std::cmp::max(max_time, t.elapsed())
        }

        s / n
    }

    fn mc_play(
        &self,
        max: Duration,
        actions: &mut Vec<A>,
        buffer: &mut Vec<A>,
        max_depth: usize,
        rng: &mut ThreadRng,
    ) -> Option<(A, f32)> {
        self.fill(actions);
        if actions.is_empty() {
            return None;
        }
        let max = Duration::from_nanos((max.as_nanos() / actions.len() as u128) as u64);
        actions
            .iter()
            .copied()
            .map(|a| (a, self.multi_run(max, a, buffer, max_depth, rng)))
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    }
}
