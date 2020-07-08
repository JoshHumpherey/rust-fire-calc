use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use rand::Rng;
use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::view::ContinuousView;
use plotlib::style::{PointMarker, PointStyle};
use plotlib::repr::*;

const STOCK_FILENAME: &str = "stocks.txt";
const BOND_FILENAME: &str = "bonds.txt";
const YEARLY_CONTRIBUTION: f64 = 60_000.0;
const TIME_HORIZON_IN_YEARS: i32 = 75;
const CONTRIBUTION_YEARS: i32 = 25;
const STIPEND: f64 = 65_000.0;
const SIMULATIONS: i32 = 50_000;
const INITIAL_CAPITAL: f64 = 135_000.0;
const STOCK_WEIGHT: f64 = 1.0;
const BOND_WEIGHT: f64 = 1.0-STOCK_WEIGHT;
const BINS: plotlib::repr::HistogramBins = plotlib::repr::HistogramBins::Count(25);

fn main() {
    let stock_hashmap = parse_data(&STOCK_FILENAME);
    let bond_hashmap = parse_data(&BOND_FILENAME);
    let simulation_results = monte_carlo_generator(&stock_hashmap, &bond_hashmap);
    plot_data(simulation_results);
}

fn plot_data(simulation_results: Vec<f64>) {
    let hist = Histogram::from_slice(&simulation_results, BINS);

    // The 'view' describes what set of data is drawn
    let v = ContinuousView::new()
        .add(hist)
        .x_label("Net Worth")
        .y_label("Frequency");
    
    // A page with a single view is then saved to an SVG file
    Page::single(&v).save("scatter.svg").unwrap();
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
    
    for i in 1..TIME_HORIZON_IN_YEARS {
        let year_index = rand::thread_rng().gen_range(0, limit);
        let stock_performance = stock_hashmap.get(&year_index).unwrap();
        let bond_performance = bond_hashmap.get(&year_index).unwrap();
        current_portfolio_value = calculate_year_performance(stock_performance, bond_performance, current_portfolio_value);
        match i <= CONTRIBUTION_YEARS {
            true => current_portfolio_value += YEARLY_CONTRIBUTION,
            false => current_portfolio_value -= STIPEND
        };
        if current_portfolio_value <= 0.0 {
            return 0.0;
        }
    }
    return current_portfolio_value;
} 

fn calculate_year_performance(stock_performance: &f64, bond_performance: &f64, current_value: f64) -> f64 {
    let stock_change = (current_value * stock_performance)*STOCK_WEIGHT;
    let bond_change = (current_value * bond_performance)*BOND_WEIGHT;
    return stock_change + bond_change + current_value;
}

fn monte_carlo_generator(stock_hashmap: &HashMap<usize, f64>, bond_hashmap: &HashMap<usize, f64>) -> Vec<f64> {
    let mut pass_count = 0.0;
    let mut fail_count = 0.0;
    let mut vec = Vec::new();

    for _ in 0..SIMULATIONS {
        let result = simulate_investor_lifetime(&stock_hashmap, &bond_hashmap);
        match result > 0.0 {
            true => pass_count = pass_count + 1.0,
            false => fail_count = fail_count + 1.0
        };
        vec.push(result);
    }
    let denominator = pass_count + fail_count;
    let pct = (pass_count / denominator)*100.0;
    println!("Out of {} simulations you passed {}% of the time", SIMULATIONS, pct);
    return vec;
}