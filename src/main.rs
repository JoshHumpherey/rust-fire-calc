use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use rand::Rng;
use gnuplot::{Figure, Caption, Color};
use std::vec;

const STOCK_FILENAME: &str = "stocks.txt";
const BOND_FILENAME: &str = "bonds.txt";
const YEARLY_CONTRIBUTION: f64 = 50_000.0;
const TIME_HORIZON_IN_YEARS: i32 = 40;
const SIMULATIONS: i32 = 100;
const INITIAL_CAPITAL: f64 = 50_000.0;
const STOCK_WEIGHT: f64 = 0.75;
const BOND_WEIGHT: f64 = 1.0-STOCK_WEIGHT;

fn main() {
    let stock_hashmap = parse_data(&STOCK_FILENAME);
    let bond_hashmap = parse_data(&BOND_FILENAME);
    let simulation_results = monte_carlo_generator(&stock_hashmap, &bond_hashmap);
    
    let mut fg = Figure::new();
    fg.axes2d()
        .lines(&simulation_results, &simulation_results, &[Caption("A line"), Color("black")]);
    fg.show().unwrap();
}

fn parse_data(filename: &str) -> HashMap<usize, f64> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut hash_map = HashMap::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        hash_map.insert(index, line.parse().unwrap());
    }

    return hash_map;
}

fn simulate_investor_lifetime(stock_hashmap: &HashMap<usize, f64>, bond_hashmap: &HashMap<usize, f64>) -> f64 {
    let limit = stock_hashmap.len();
    let mut current_portfolio_value = INITIAL_CAPITAL;
    for _ in 0..TIME_HORIZON_IN_YEARS {
        let year_index = rand::thread_rng().gen_range(0, limit);
        let stock_performance = stock_hashmap.get(&year_index).unwrap();
        let bond_performance = bond_hashmap.get(&year_index).unwrap();
        current_portfolio_value = calculate_year_performance(stock_performance, bond_performance, current_portfolio_value);
    }
    return current_portfolio_value;
} 

fn calculate_year_performance(stock_performance: &f64, bond_performance: &f64, current_value: f64) -> f64 {
    let stock_change = (current_value * stock_performance)*STOCK_WEIGHT;
    let bond_change = (current_value * bond_performance)*BOND_WEIGHT;
    return stock_change + bond_change + current_value + YEARLY_CONTRIBUTION;
}

fn monte_carlo_generator(stock_hashmap: &HashMap<usize, f64>, bond_hashmap: &HashMap<usize, f64>) -> Vec<f64>{
    let mut vec = Vec::new();
    for _ in 0..SIMULATIONS {
        let result = simulate_investor_lifetime(&stock_hashmap, &bond_hashmap);
        vec.push(result);
    }
    return vec;
}