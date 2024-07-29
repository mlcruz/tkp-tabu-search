use highs::{HighsModelStatus, RowProblem, Sense};

use crate::parse::TkpInstance;

impl TkpInstance {
    pub fn higgs_solve(&self) -> i32 {
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
                .zip(x.iter())
                .filter(|(order, _)| order.start <= t && order.end >= t)
                .map(|(order, x)| (*x, order.demand as f64))
                .collect();

            // se a some das demandas das ordens ativas em t for menor ou igual a capacidade
            // não precisamos de restrição (todas as variaveis podem estar ligadas)
            let sum_demands = interval_orders
                .iter()
                .map(|(_, demand)| demand)
                .sum::<f64>();

            if interval_orders.is_empty() || self.c as f64 >= sum_demands {
                continue;
            }

            // o bound maximo é a capacidade de c
            pb.add_row(0f64..=self.c as f64, interval_orders)
        }

        println!(
            "solving {} constrants, {} variables",
            pb.num_rows(),
            pb.num_cols()
        );

        let mut model = pb.optimise(Sense::Maximise);

        // usar o solver simplex é significativamente mais rapido
        model.set_option("presolve", "on");

        if self.orders.len() > 1000 {
            model.set_option("solver", "simplex");
        }
        model.set_option("parallel", "on");

        let solved = model.solve();

        assert_eq!(solved.status(), HighsModelStatus::Optimal);
        let solution = solved.get_solution();

        // escreve arquivo com a solução (para debug)
        std::fs::write(
            format!("{}.sol", self.name),
            solution
                .columns()
                .iter()
                .map(|&val| (val.round() as i32).to_string())
                .collect::<Vec<_>>()
                .join("\n"),
        )
        .unwrap();

        // calcula o valor da solução, somand o lucro das ordens selecionadas
        let objective_value: i32 = solution
            .columns()
            .iter()
            .enumerate()
            .map(|(i, &val)| (val.round() as i32) * self.orders[i].profit as i32)
            .sum();

        return objective_value;
    }
}
