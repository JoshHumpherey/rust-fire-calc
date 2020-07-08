use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use rand::Rng;
use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::view::ContinuousView;
use plotlib::style::{PointMarker, PointStyle};

const STOCK_FILENAME: &str = "stocks.txt";
const BOND_FILENAME: &str = "bonds.txt";
const YEARLY_CONTRIBUTION: f64 = 60_000.0;
const TIME_HORIZON_IN_YEARS: i32 = 25;
const SIMULATIONS: i32 = 1;
const INITIAL_CAPITAL: f64 = 135_000.0;
const STOCK_WEIGHT: f64 = 0.75;
const BOND_WEIGHT: f64 = 1.0-STOCK_WEIGHT;

fn main() {
    let stock_hashmap = parse_data(&STOCK_FILENAME);
    let bond_hashmap = parse_data(&BOND_FILENAME);
    let simulation_results = monte_carlo_generator(&stock_hashmap, &bond_hashmap);
    plot_data(simulation_results);

}

fn plot_data(simulation_results: Vec<Vec<(f64, f64)>>) {
    let test_data = simulation_results.get(0).unwrap();
    let s1: Plot = Plot::new(test_data.to_vec()).point_style(
        PointStyle::new()
            .marker(PointMarker::Circle) // setting the marker to be a circle
            .colour("#DD3355"),
    ); // and a custom colour

    // The 'view' describes what set of data is drawn
    let v = ContinuousView::new()
        .add(s1)
        .x_label("Investing Timeline")
        .y_label("Net Worth");
    
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

fn simulate_investor_lifetime(stock_hashmap: &HashMap<usize, f64>, bond_hashmap: &HashMap<usize, f64>) -> Vec<(f64, f64)> {
    let limit = stock_hashmap.len();
    let mut current_portfolio_value = INITIAL_CAPITAL;
    let mut investor_lifetime = Vec::new();
    investor_lifetime.push((0.0, current_portfolio_value));

    for i in 1..TIME_HORIZON_IN_YEARS {
        let year_index = rand::thread_rng().gen_range(0, limit);
        let stock_performance = stock_hashmap.get(&year_index).unwrap();
        let bond_performance = bond_hashmap.get(&year_index).unwrap();
        current_portfolio_value = calculate_year_performance(stock_performance, bond_performance, current_portfolio_value);
        let tuple = (i as f64, current_portfolio_value);
        investor_lifetime.push(tuple);
    }
    return investor_lifetime;
} 

fn calculate_year_performance(stock_performance: &f64, bond_performance: &f64, current_value: f64) -> f64 {
    let stock_change = (current_value * stock_performance)*STOCK_WEIGHT;
    let bond_change = (current_value * bond_performance)*BOND_WEIGHT;
    return stock_change + bond_change + current_value + YEARLY_CONTRIBUTION;
}

fn monte_carlo_generator(stock_hashmap: &HashMap<usize, f64>, bond_hashmap: &HashMap<usize, f64>) -> Vec<Vec<(f64, f64)>> {
    let mut simulation_results = Vec::new();

    for _ in 0..SIMULATIONS {
        let result = simulate_investor_lifetime(&stock_hashmap, &bond_hashmap);
        simulation_results.push(result);
    }

    return simulation_results;
}