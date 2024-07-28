use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use rand::SeedableRng;

#[derive(Debug, Clone)]
pub struct TkpInstance {
    pub n: usize,
    pub c: u32,
    pub orders: Vec<Order>,
    pub name: String,
    pub rng: rand::rngs::StdRng,
}

#[derive(Debug, Clone, Copy)]
pub struct Order {
    pub profit: u32,
    pub demand: u32,
    pub start: u32,
    pub end: u32,
}

impl Order {
    pub fn parse_from_line(line: &str) -> Self {
        let values: Vec<u32> = line
            .trim()
            .split(" ")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        Order {
            profit: values[0],
            demand: values[1],
            start: values[2],
            end: values[3],
        }
    }
}

impl TkpInstance {
    pub fn parse_from_file(path: &Path, seed: u64) -> Self {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let mut lines = reader.lines();
        let n = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
        let c = lines.next().unwrap().unwrap().parse::<u32>().unwrap();
        let orders = lines
            .take(n)
            .map(|x| Order::parse_from_line(&x.unwrap()))
            .collect::<Vec<_>>();

        TkpInstance {
            n: n.try_into().unwrap(),
            c,
            orders,
            name: path.file_stem().unwrap().to_str().unwrap().to_string(),
            rng: rand::rngs::StdRng::seed_from_u64(seed),
        }
    }

    pub fn parse_instance_folder(path: &Path, seed: u64) -> Vec<Self> {
        let mut instances = Vec::new();
        let paths = std::fs::read_dir(path).unwrap();
        for path in paths {
            let path = path.unwrap().path();
            if path.is_file() {
                instances.push(TkpInstance::parse_from_file(&path, seed));
            }
        }
        instances
    }
}
