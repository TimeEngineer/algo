use rand::{
    prelude::{SliceRandom, ThreadRng},
    Rng,
};
use std::{
    fmt::Debug,
    time::{Duration, Instant},
};

/// GA stands for Genetic Algorithm
/// - A: Action
/// - CHR_SIZE: Chromosome size
/// - POP_SIZE: Population size (> 10)
/// - SEL_SIZE: Population size kept (< POP_SIZE)
pub trait Ga<A: Debug + Default + Copy, const CHR_SIZE: usize>: Debug + Clone {
    const POP_SIZE: usize;
    const SEL_SIZE: usize;

    /// Generate all possible alleles for each gene
    fn fill(&self, possible_genes: &mut [Vec<A>; CHR_SIZE]);

    /// Heuristic evaluation
    fn eval(&self, chromosome: &[A; CHR_SIZE]) -> i64;

    fn clear_and_fill(&self, possible_genes: &mut [Vec<A>; CHR_SIZE]) {
        possible_genes.iter_mut().for_each(Vec::clear);
        self.fill(possible_genes);
    }

    fn gen(rng: &mut ThreadRng, possible_genes: &mut [Vec<A>; CHR_SIZE]) -> [A; CHR_SIZE] {
        let mut chromosome = [A::default(); CHR_SIZE];
        for i in 0..CHR_SIZE {
            chromosome[i] = *possible_genes[i].choose(rng).unwrap_or(&A::default());
        }

        chromosome
    }

    fn mate(
        rng: &mut ThreadRng,
        possible_genes: &[Vec<A>; CHR_SIZE],
        chromosome0: &[A; CHR_SIZE],
        chromosome1: &[A; CHR_SIZE],
    ) -> [A; CHR_SIZE] {
        let mut chromosome = [A::default(); CHR_SIZE];
        for (i, (gene0, gene1)) in chromosome0.iter().zip(chromosome1).enumerate() {
            // 126/256 ~ 0.492
            // 4/256 ~ 0.016
            chromosome[i] = match rng.gen::<u8>() {
                0..=125 => *gene0,
                126..=251 => *gene1,
                252..=255 => *possible_genes[i].choose(rng).unwrap_or(&A::default()),
            }
        }

        chromosome
    }

    fn ga(
        &mut self,
        rng: &mut ThreadRng,
        population: &mut Vec<(i64, [A; CHR_SIZE])>,
        possible_genes: &mut [Vec<A>; CHR_SIZE],
        total_time: Duration,
    ) -> (i64, [A; CHR_SIZE]) {
        let now = Instant::now();
        let mut max_time = Duration::default();

        self.clear_and_fill(possible_genes);

        // Can't build chromosome without choice
        if possible_genes.iter().map(Vec::len).sum::<usize>() == 0 {
            return (0, [A::default(); CHR_SIZE]);
        }

        population.reserve(Self::POP_SIZE);
        for _ in 0..Self::POP_SIZE {
            population.push((0, Self::gen(rng, possible_genes)))
        }

        // Sort in reversed way, best must be first
        population
            .iter_mut()
            .for_each(|(score, chr)| *score = self.eval(chr));
        population.sort_by_key(|(score, _)| std::cmp::Reverse(*score));

        while now.elapsed() + max_time < total_time {
            let t = Instant::now();

            // Keep the 10% best chromosomes
            population.truncate(10 * Self::POP_SIZE / 100);

            // Perform crossover & mutation
            for _ in population.len()..Self::POP_SIZE {
                let chromosome0 = population.choose(rng).unwrap().1;
                let chromosome1 = population.choose(rng).unwrap().1;
                population.push((0, Self::mate(rng, possible_genes, &chromosome0, &chromosome1)))
            }

            // Order
            population
                .iter_mut()
                .for_each(|(score, chr)| *score = self.eval(chr));
            population.sort_by_key(|(score, _)| std::cmp::Reverse(*score));

            max_time = std::cmp::max(max_time, t.elapsed());
        }

        *population[..Self::SEL_SIZE].choose(rng).unwrap()
    }
}
