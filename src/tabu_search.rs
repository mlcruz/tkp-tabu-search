use rand::{seq::IteratorRandom, Rng};
use std::collections::{BTreeMap, HashSet, VecDeque};

use crate::parse::TkpInstance;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Solution {
    pub selected_orders: Vec<bool>,
    pub total_profit: u32,
    pub is_feasible: bool,
    total_demand: Vec<u32>,
}

impl Solution {
    fn new(size: usize, profit: u32, last_order_end: usize) -> Self {
        Self {
            selected_orders: vec![false; size],
            total_profit: profit,
            is_feasible: true,
            total_demand: vec![0; last_order_end],
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
    pub selected_for_profit_pool: Vec<usize>,
    pub selected_for_slack_fill: Vec<usize>,
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
            selected_for_profit_pool: Vec::new(),
            selected_for_slack_fill: Vec::new(),
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
        let last_order_end = self
            .tkp_instance
            .orders
            .iter()
            .map(|x| x.end)
            .max()
            .unwrap();

        let mut best_solution =
            Solution::new(self.tkp_instance.orders.len(), 0, last_order_end as usize);
        let mut current_solution = best_solution.clone();

        for _ in 0..iterations {
            let neighbors: Vec<Solution> = (0..self.neighborhood_size)
                .map(|_| self.generate_neighbor(&current_solution))
                .collect();

            let feasible_neighbors: Vec<Solution> = neighbors
                .into_iter()
                .filter(|neighbor| !self.is_tabu(neighbor) && neighbor.is_feasible)
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

            self.selected_for_profit_pool.clear();
            self.selected_for_slack_fill.clear();
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
            .filter(|(_, idx)| {
                !current_solution.selected_orders[**idx]
                    && !self.selected_for_profit_pool.contains(idx)
            })
            .filter(|(_, idx)| {
                // seleciona apenas ordens que não ultrapassam a capacidade
                let order = &self.tkp_instance.orders[**idx];
                let mut is_feasible = true;

                for t in order.start..=order.end {
                    let period_index = (t - 1) as usize;

                    if current_solution.total_demand[period_index] + order.demand
                        > self.tkp_instance.c
                    {
                        is_feasible = false;
                        break;
                    }
                }

                is_feasible
            })
            .map(|(_, idx)| idx)
            // seleciona uma das 5 melhores opções
            .take(5)
            .choose(&mut self.tkp_instance.rng);

        if selected.is_none() {
            return self.generate_random_neighbor(current_solution);
        }

        let selected_idx = selected.unwrap();

        self.selected_for_profit_pool.push(*selected_idx);

        let mut neighbor = current_solution.clone();
        neighbor.is_feasible = true;
        neighbor.selected_orders[*selected_idx] = true;
        neighbor.total_profit += self.tkp_instance.orders[*selected_idx].profit as u32;

        self.update_neighbor_total_demand(neighbor, *selected_idx, true, false)
    }

    // heuristica: gera vizinhança de soluções levando em consideração
    // a maior quantidade de capacide livre preenchida
    fn generate_best_profit_slack_fill(&mut self, current_solution: &Solution) -> Solution {
        let slack = current_solution
            .total_demand
            .iter()
            .map(|x| self.tkp_instance.c - x)
            .collect::<Vec<_>>();

        let mut orders_by_slack = self
            .tkp_instance
            .orders
            .iter()
            .enumerate()
            .filter(|(idx, _)| {
                !current_solution.selected_orders[*idx]
                    && !self.selected_for_slack_fill.contains(idx)
            })
            .filter(|(idx, _)| {
                // seleciona apenas ordens que não ultrapassam a capacidade
                let order = &self.tkp_instance.orders[*idx];
                let mut is_feasible = true;

                for t in order.start..=order.end {
                    let period_index = (t - 1) as usize;

                    if current_solution.total_demand[period_index] + order.demand
                        > self.tkp_instance.c
                    {
                        is_feasible = false;
                        break;
                    }
                }

                is_feasible
            })
            .map(|(i, x)| {
                let total_order_slack_fill = (x.start..=x.end)
                    .map(|t| slack[(t - 1) as usize])
                    .sum::<u32>();

                (i, total_order_slack_fill)
            })
            .collect::<Vec<_>>();

        orders_by_slack.sort_unstable_by_key(|x| x.1);

        let selected = orders_by_slack
            .iter()
            .rev()
            .take(5)
            .choose(&mut self.tkp_instance.rng);

        if selected.is_none() {
            return self.generate_random_neighbor(current_solution);
        }

        let selected_idx = selected.unwrap().0;
        self.selected_for_slack_fill.push(selected_idx);

        let mut neighbor = current_solution.clone();
        neighbor.is_feasible = true;

        neighbor.selected_orders[selected_idx] = true;
        neighbor.total_profit += self.tkp_instance.orders[selected_idx].profit as u32;
        self.update_neighbor_total_demand(neighbor, selected_idx, true, false)
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

        self.update_neighbor_total_demand(neighbor, idx, false, true)
    }

    fn update_neighbor_total_demand(
        &mut self,
        mut neighbor: Solution,
        idx: usize,
        should_be_feasible: bool,
        subtract: bool,
    ) -> Solution {
        neighbor.is_feasible = true;

        for t in self.tkp_instance.orders[idx].start..=self.tkp_instance.orders[idx].end {
            let period_index = (t - 1) as usize;

            if subtract {
                neighbor.total_demand[period_index] -= self.tkp_instance.orders[idx].demand;
            } else {
                neighbor.total_demand[period_index] += self.tkp_instance.orders[idx].demand;
            }
            if neighbor.total_demand[period_index] as u32 > self.tkp_instance.c {
                neighbor.is_feasible = false;
                break;
            }
        }

        if should_be_feasible {
            assert_eq!(neighbor.is_feasible, true);
        }

        neighbor
    }

    fn generate_neighbor(&mut self, current_solution: &Solution) -> Solution {
        let random_strategy = self.tkp_instance.rng.gen_range(0..=2);

        match random_strategy {
            0 => self.generate_random_neighbor(current_solution),
            1 => self.generate_best_profit_pool(current_solution),
            2 => self.generate_best_profit_slack_fill(current_solution),
            _ => unreachable!(),
        }
    }
}
