mod parse;
mod tabu_search;

fn main() {
    let instances =
        parse::TkpInstance::parse_instance_folder(std::path::Path::new("tkp_instances"));

    for instance in instances {}
}
