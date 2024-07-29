use parse::TkpInstance;

mod higgs_solve;
mod parse;
mod tabu_search;
fn main() {
    let seed: u64 = rand::random();
    let instances =
        parse::TkpInstance::parse_instance_folder(std::path::Path::new("tkp_instances"), seed);

    for instance in instances {
        let mut now = std::time::Instant::now();

        //let higgs_solution = instance.higgs_solve();
        // println!(
        //     "{}: HIGHS: {}, time: {}ms",
        //     instance.name,
        //     higgs_solution,
        //     now.elapsed().as_millis()
        // );
        tabu_scenarios(instance)
    }
}

fn tabu_scenarios(instance: TkpInstance) {
    println!("{}\titerations\ttabu_list_size\tneighborhood_size\tdisable_cost_benefit\tdisable_slack_fill\ttotal_profit\ttime", instance.name);

    let iterations = [1000, 5000, 10000, 20000];
    let slack_fill = [true, false];
    let cost_benefit = [true, false];
    let tabu_list_size = [10, 20, 30];
    let neighborhood_size = [5, 10, 20];

    for iterations in iterations.iter() {
        for slack_fill in slack_fill.iter() {
            for cost_benefit in cost_benefit.iter() {
                for tabu_list_size in tabu_list_size.iter() {
                    for neighborhood_size in neighborhood_size.iter() {
                        instance.tabu_search(
                            *iterations,
                            *tabu_list_size,
                            *neighborhood_size,
                            *cost_benefit,
                            *slack_fill,
                        );
                    }
                }
            }
        }
    }
}
