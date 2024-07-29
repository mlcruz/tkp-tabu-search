mod higgs_solve;
mod parse;
mod tabu_search;
fn main() {
    let seed: u64 = rand::random();
    let instances =
        parse::TkpInstance::parse_instance_folder(std::path::Path::new("tkp_instances"), seed);

    for instance in instances {
        let mut now = std::time::Instant::now();

        let higgs_solution = instance.higgs_solve();
        println!(
            "{}: HIGHS: {}, time: {}ms",
            instance.name,
            higgs_solution,
            now.elapsed().as_millis()
        );

        now = std::time::Instant::now();

        let tabu_solution = instance.tabu_search(1000, 20, 10);

        println!(
            "{}: TABU: {}, time:{}ms",
            instance.name,
            tabu_solution.total_profit,
            now.elapsed().as_millis()
        );
    }
}
