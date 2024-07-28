mod higgs_solve;
mod parse;
mod tabu_search;

fn main() {
    let instances =
        parse::TkpInstance::parse_instance_folder(std::path::Path::new("tkp_instances"));

    for instance in instances {
        let higgs_solution = instance.higgs_solve();
        let first_line = &instance.orders[0];
        let last_line = &instance.orders[instance.orders.len() - 1];
        println!(
            "{}: {} {} {} {}",
            instance.name, first_line.profit, first_line.demand, first_line.start, first_line.end
        );

        println!(
            "{}: {} {} {} {}",
            instance.name, last_line.profit, last_line.demand, last_line.start, last_line.end
        );

        println!("{}: HIGHS: {}", instance.name, higgs_solution);
    }
}
