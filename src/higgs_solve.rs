use highs::{HighsModelStatus, RowProblem, Sense};

use crate::parse::TkpInstance;

impl TkpInstance {
    pub fn higghs_solve(&self) -> f64 {
        let mut pb = RowProblem::default();

        // X representa uma variavel binaria representando se uma ordem é escolhida ou não
        // com seu lucro como coeficiente
        let x: Vec<_> = self
            .orders
            .iter()
            .map(|o| pb.add_integer_column(o.profit as f64, 0..=1))
            .collect();

        // Tempo do fim da ordem mais tardia (representa o T final do problema)
        let final_time = self.orders.iter().map(|o| o.end).max().unwrap_or(0);

        // Para cada segundo do intervalo total
        // a soma das demandas das ordens selecionadas devem ser menor ou igual a capacidade
        for t in 0..=final_time {
            // ordens que estao ativas em t, zipadas com suas respectivas variaveis de seleção
            let interval_orders: Vec<_> = self
                .orders
                .iter()
                .enumerate()
                .filter(|(_, order)| order.start <= t && order.end >= t)
                .map(|(i, order)| (x[i], order.demand as f64))
                .collect();

            // o bound maximo poderia ser ou a capacidade de t, ou a soma
            // das demandas das ordens ativas em t.
            let sum_of_demands: f64 = interval_orders.iter().map(|(_, o)| o).sum();
            pb.add_row(0..=self.c as i32, interval_orders)
        }

        println!(
            "solving {} constrants, {} variables",
            pb.num_rows(),
            pb.num_cols()
        );
        // Set the optimization sense to maximize the objective function
        let solved = pb.optimise(Sense::Maximise).solve();

        assert_eq!(solved.status(), HighsModelStatus::Optimal);
        let solution = solved.get_solution();
        let objective_value: f64 = solution
            .columns()
            .iter()
            .enumerate()
            .map(|(i, &val)| val * self.orders[i].profit as f64)
            .sum();

        return objective_value;
    }
}
