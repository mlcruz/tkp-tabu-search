mod higgs_solve;
mod parse;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let path = args.get(1).unwrap();
    let path = std::path::Path::new(path);

    let seed = args.get(2).unwrap().parse::<u64>().unwrap();
    let instance = parse::TkpInstance::parse_from_file(path, seed);

    let now = std::time::Instant::now();
    let highs_solution = instance.highs_solve();
    println!("name,profit,time",);
    println!(
        "{},{},{}ms",
        instance.name,
        highs_solution,
        now.elapsed().as_millis()
    );
}
