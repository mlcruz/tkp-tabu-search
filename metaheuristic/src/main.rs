mod parse;
mod tabu_search;
fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let path = args.get(1).unwrap();
    let path = std::path::Path::new(path);
    let seed = args.get(2).unwrap().parse::<u64>().unwrap();
    let iterations = args.get(3).unwrap().parse::<usize>().unwrap();

    let tabu_list_size = args.get(4).unwrap().parse::<usize>().unwrap();
    let neighborhood_size = args.get(5).unwrap().parse::<usize>().unwrap();

    let instance = parse::TkpInstance::parse_from_file(path, seed);

    println!("name,seed,iterations,tabu_list_size,neighborhood_size,total_profit,time");
    instance.tabu_search(iterations, tabu_list_size, neighborhood_size, seed);
}
