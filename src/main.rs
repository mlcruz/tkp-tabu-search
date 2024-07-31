use parse::TkpInstance;

mod higgs_solve;
mod parse;
mod tabu_search;
fn main() {
    let seed: u64 = rand::random();
    let instances =
        parse::TkpInstance::parse_instance_folder(std::path::Path::new("tkp_instances"), seed);

    for instance in instances {
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
    println!("name,seed,iterations,tabu_list_size,neighborhood_size,total_profit,time");

    let iterations = [5000, 20000];
    let random_seeds = [rand::random::<u64>(), rand::random::<u64>()];
    let tabu_list_size = [10, 50];
    let neighborhood_size = [10, 50];

    for iterations in iterations.iter() {
        for random_seed in random_seeds.iter() {
            for tabu_list_size in tabu_list_size.iter() {
                for neighborhood_size in neighborhood_size.iter() {
                    instance.tabu_search(
                        *iterations,
                        *tabu_list_size,
                        *neighborhood_size,
                        *random_seed,
                    );
                }
            }
        }
    }
}
