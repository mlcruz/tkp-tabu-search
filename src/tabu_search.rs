use rand::{seq::IteratorRandom, Rng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::collections::{BTreeMap, HashSet, VecDeque};

use crate::parse::TkpInstance;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Solution {
    pub selected_orders: Vec<bool>,
    pub total_profit: u32,
}

impl Solution {
    fn new(size: usize, profit: u32) -> Self {
        Self {
            selected_orders: vec![false; size],
            total_profit: profit,
        }
    }
}

struct TabuSearch {
    tabu_set: HashSet<Solution>,
    tabu_list: VecDeque<Solution>,
    tabu_list_size: usize,
    tkp_instance: TkpInstance,
    neighborhood_size: usize,
    pub cost_benefit: BTreeMap<u32, usize>,
}

impl TkpInstance {
    pub fn tabu_search(
        &self,
        iterations: usize,
        tabu_list_size: usize,
        neighborhood_size: usize,
    ) -> Solution {
        let cloned = self.clone();
        let mut tabu_search = TabuSearch::new(tabu_list_size, neighborhood_size, cloned);
        tabu_search.tabu_search(iterations)
    }
}

impl TabuSearch {
    fn new(tabu_list_size: usize, neighborhood_size: usize, tkp_instance: TkpInstance) -> Self {
        let cost_benefit = tkp_instance.orders.iter().enumerate().map(|(i, x)| {
            (
                (x.profit as f32 / (x.end - x.start) as f32).round() as u32,
                i,
            )
        });

        Self {
            tabu_set: HashSet::with_capacity(tabu_list_size),
            tabu_list: VecDeque::with_capacity(tabu_list_size),
            tabu_list_size,
            tkp_instance: tkp_instance.clone(),
            cost_benefit: cost_benefit.collect(),
            neighborhood_size,
        }
    }

    fn is_tabu(&self, solution: &Solution) -> bool {
        self.tabu_set.contains(solution)
    }

    fn add_to_tabu_list(&mut self, solution: Solution) {
        if self.tabu_list.len() == self.tabu_list_size {
            if let Some(old_solution) = self.tabu_list.pop_front() {
                self.tabu_set.remove(&old_solution);
            }
        }
        self.tabu_list.push_back(solution.clone());
        self.tabu_set.insert(solution);
    }

    pub fn tabu_search(&mut self, iterations: usize) -> Solution {
        let mut best_solution = Solution::new(self.tkp_instance.orders.len(), 0);
        let mut current_solution = best_solution.clone();

        for _ in 0..iterations {
            let neighbors: Vec<Solution> = (0..self.neighborhood_size)
                .map(|_| self.generate_neighbor(&current_solution))
                .collect();

            let feasible_neighbors: Vec<Solution> = neighbors
                .into_par_iter()
                .filter(|neighbor| !self.is_tabu(neighbor) && self.is_feasible(neighbor))
                .collect();

            if let Some(best_neighbor) = feasible_neighbors
                .into_iter()
                .max_by_key(|neighbor| neighbor.total_profit)
            {
                current_solution = best_neighbor.clone();
                if current_solution.total_profit > best_solution.total_profit {
                    best_solution = current_solution.clone();
                }
                self.add_to_tabu_list(current_solution.clone());
            }
        }

        best_solution
    }

    // heuristica: gera vizinhança de soluções levando em consideração
    // o valor duração/lucro de pedidos ainda não selecionados
    fn generate_best_profit_pool(&mut self, current_solution: &Solution) -> Solution {
        let selected = self
            .cost_benefit
            .iter()
            .rev()
            .filter(|(_, idx)| !current_solution.selected_orders[**idx])
            .map(|(_, idx)| idx)
            .take(50)
            .choose(&mut self.tkp_instance.rng);

        if selected.is_none() {
            return self.generate_random_neighbor(current_solution);
        }

        let selected = selected.unwrap();
        let mut neighbor = current_solution.clone();
        neighbor.selected_orders[*selected] = true;
        neighbor.total_profit += self.tkp_instance.orders[*selected].profit as u32;

        neighbor
    }

    fn generate_random_neighbor(&mut self, current_solution: &Solution) -> Solution {
        let mut neighbor = current_solution.clone();
        let idx = self
            .tkp_instance
            .rng
            .gen_range(0..self.tkp_instance.orders.len());

        // se selecionado, deseleciona e calcula lucro total
        if neighbor.selected_orders[idx] {
            neighbor.selected_orders[idx] = false;
            neighbor.total_profit =
                current_solution.total_profit - self.tkp_instance.orders[idx].profit as u32;
            return neighbor;
        }

        // se deselecionado, seleciona e calcula lucro total
        neighbor.selected_orders[idx] = true;
        neighbor.total_profit =
            current_solution.total_profit + self.tkp_instance.orders[idx].profit as u32;
        neighbor
    }

    fn generate_neighbor(&mut self, current_solution: &Solution) -> Solution {
        let random_strategy = self.tkp_instance.rng.gen_range(0..=1);

        if random_strategy == 0 {
            self.generate_random_neighbor(current_solution)
        } else {
            self.generate_best_profit_pool(current_solution)
        }
    }

    fn is_feasible(&self, solution: &Solution) -> bool {
        let mut total_demand = vec![
            0;
            (self
                .tkp_instance
                .orders
                .iter()
                .map(|o| o.end)
                .max()
                .unwrap_or(0)
                + 1) as usize
        ];

        for (i, selected) in solution.selected_orders.iter().enumerate() {
            if *selected {
                let order = &self.tkp_instance.orders[i];
                for t in order.start..=order.end {
                    total_demand[t as usize] += order.demand;
                    if total_demand[t as usize] > self.tkp_instance.c {
                        return false; // Infeasible solution
                    }
                }
            }
        }
        true
    }
}
