use rand::{prelude::ThreadRng, seq::SliceRandom};
use std::{
    fmt::Debug,
    time::{Duration, Instant},
};

pub trait MctsFlow<A: Debug + Copy>: Clone {
    /// Generate all possible actions
    fn fill(&self, actions: &mut Vec<A>);
    /// Update the system
    fn update(&mut self, action: A) -> i32;

    fn clear_and_fill(&self, actions: &mut Vec<A>) {
        actions.clear();
        self.fill(actions)
    }

    fn run(
        &self,
        rng: &mut ThreadRng,
        action: A,
        actions: &mut Vec<A>,
    ) -> i32 {
        let mut run = self.clone();
        let mut score = run.update(action);
        run.clear_and_fill(actions);

        while let Some(action) = actions.choose(rng) {
            score += run.update(*action);
            run.clear_and_fill(actions);
        }

        score
    }

    fn multi_run(
        &self,
        rng: &mut ThreadRng,
        max: Duration,
        action: A,
        actions: &mut Vec<A>,
    ) -> (i32, usize) {
        let now = Instant::now();
        let mut max_time = Duration::default();
        let mut s = 0;
        let mut n = 0;

        while now.elapsed() + max_time < max {
            let t = Instant::now();
            s += self.run(rng, action, actions);
            n += 1;
            max_time = std::cmp::max(max_time, t.elapsed())
        }

        (s, n)
    }

    fn mcts(
        &self,
        rng: &mut ThreadRng,
        max_time: Duration,
        actions: &mut Vec<A>,
        buffer: &mut Vec<A>,
        scores: &mut Vec<(i32, usize)>,
    ) -> Option<(A, f32)> {
        self.clear_and_fill(actions);
        if actions.is_empty() {
            return None;
        }
        let max_time = Duration::from_nanos((max_time.as_nanos() / actions.len() as u128) as u64);

        scores.clear();

        actions
            .iter()
            .copied()
            .for_each(|a| scores.push(self.multi_run(rng, max_time, a, buffer)));

        let n_tot = scores.iter().map(|(_, n)| n).sum::<usize>() as f32;

        actions
            .iter()
            .copied()
            .zip(scores.iter().copied())
            .map(|(a, (s, n))| {
                let n = n as f32;
                let s = s as f32;
                (a, s / n + (2. * n_tot.ln() / n).sqrt())
            })
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    }
}