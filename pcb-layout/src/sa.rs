use crate::ga;
//use crate::plcmnt;
use colored::Colorize;
use ga::Individual;
use rand::prelude::*;
use std::time::Instant;
struct ParetoFront<'a>{
    front:Vec<&'a  Individual>,
}
impl<'a> ParetoFront<'a> {
    fn check_and_clean(&mut self, ind: &'a  Individual){
        let cur_hpwl = ind.hpwl_score();
        let cur_area = ind.area_score();
        let mut should_add = self.front.len() == 0;
        let mut to_remove: Vec<& Individual> = Vec::new();
        for i in &self.front{
            if i.hpwl_score() > cur_hpwl{
                should_add = true;
            }
            if i.area_score() > cur_area{
                if should_add{
                    //i is dominated
                    to_remove.push(i);
                }
                should_add = true;
            }
        }
        if should_add{
            self.front.push(ind);
        }

    }
}
pub fn log_cool(start_temp: f64, iter: i32) -> f64 {
    //C / log(1 + i)
    start_temp / ((1.0 + iter as f64).log2())
}
#[allow(dead_code)]
pub fn linear_cool(start_temp: f64, iter: i32) -> f64 {
    start_temp - iter as f64 * 0.001
}
pub fn quick_sa(
    mut ind: Individual,
    cooling_schedule: fn(f64, i32) -> f64,
    max_count: i32,
    print_time: bool,
) -> Individual {
    let mut rng = rand::rng();
    let start_temp = 100.0;
    let mut temp = start_temp;
    let now = Instant::now();
    let mut old_fit = ind.score();
    //Go to stop
    let mut count = 0;
    if print_time {
        println!(
            "{}",
            format!(
                "+++++++Test (Simulated Annealing , Max Iterations: {}) +++++++",
                max_count
            )
            .green()
        );
        println!("Original Score: {}", ind.score());
    }

    while temp > 0.0 && count < max_count {
        let mut new_ind = ind.clone();
        if new_ind.mutate(&mut rng) {
            //Compare fitness
            let new_fit = new_ind.score();
            let fitness_diff = old_fit / new_fit;
            let acceptance_odds = fitness_diff * (temp / start_temp);
            //println!("Iter {}: temp: {}, old_fitness: {}, new fitness: {}, fitness_diff: {}, odds:{}", count, temp, old_fit , new_fit, fitness_diff, acceptance_odds);
            if acceptance_odds > rng.random_range(0.0..=1.0) {
                ind = new_ind;
                old_fit = new_fit;
            }
        }
        temp = cooling_schedule(start_temp, count);
        count += 1;
    }
    if print_time {
        println!("New Score: {}", ind.score());
        let elapsed_time = now.elapsed();
        println!(
            "\n{} iterations took {}.{} seconds.",
            count,
            elapsed_time.as_secs(),
            elapsed_time.subsec_millis()
        );
        println!("\n{}", "+++++++Test Over+++++++".to_string().green());
    }
    ind
}

pub fn pareto_sa(
    mut ind: Individual,
    cooling_schedule: fn(f64, i32) -> f64,
    max_count: i32,
    print_time: bool,
) -> Individual {
    let mut rng = rand::rng();
    let start_temp = 100.0;
    let mut temp = start_temp;
    let now = Instant::now();
    let mut old_fit = ind.score();
    //Go to stop
    let mut count = 0;
    let mut p_front =  ParetoFront{front: Vec::new()};
    if print_time {
        println!(
            "{}",
            format!(
                "+++++++Test (Simulated Annealing , Max Iterations: {}) +++++++",
                max_count
            )
            .green()
        );
        println!("Original Score: {}", ind.score());
    }
    
    while temp > 0.0 && count < max_count {
        let mut this_run = Vec::new();
        for ind in &p_front.front{
            let mut new_ind = (*ind).clone();
            if new_ind.mutate(&mut rng) {
                //Compare fitness
                let new_fit = new_ind.score();
                let fitness_diff = old_fit / new_fit;
                let acceptance_odds = fitness_diff * (temp / start_temp);
                //println!("Iter {}: temp: {}, old_fitness: {}, new fitness: {}, fitness_diff: {}, odds:{}", count, temp, old_fit , new_fit, fitness_diff, acceptance_odds);
                if acceptance_odds > rng.random_range(0.0..=1.0) {
                    
                    old_fit = new_fit;
                    this_run.push(new_ind);
                }
            }
        }
        for ind in &this_run{
            p_front.check_and_clean( ind);
        }
        temp = cooling_schedule(start_temp, count);
        count += 1;
    }
    if print_time {
        println!("New Score: {}", ind.score());
        let elapsed_time = now.elapsed();
        println!(
            "\n{} iterations took {}.{} seconds.",
            count,
            elapsed_time.as_secs(),
            elapsed_time.subsec_millis()
        );
        println!("\n{}", "+++++++Test Over+++++++".to_string().green());
    }
    ind
}
