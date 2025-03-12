use crate::ga;
use ga::{elitist_selection, ev_selection, generate_animation, genetic_algorithim, Individual};
use crate::plcmnt;
use plcmnt::{Bbox, Component, Pin, Placement};
use rand::prelude::*;



fn log_cool(temp:f64) -> f64{
0.0
}
pub fn linear_cool(temp:f64) -> f64{
    temp - (temp / 100.0)
}
pub fn quick_sa(mut ind: Individual, cooling_schedule : fn(f64) -> f64) -> Individual{
    let mut rng = rand::rng();
    let start_temp = 100.0;
    let mut temp = start_temp;

    let mut old_fit = ind.score();
    //Go to stop
    let mut count = 0;
    while temp > 10.0{
        let mut new_ind = ind.clone();
        if new_ind.mutate(&mut rng)
        {
            //Compare fitness
            let new_fit = new_ind.score();
            let fitness_diff = old_fit/new_fit;
            let acceptance_odds = fitness_diff * (temp/start_temp);
            //println!("Iter {}: temp: {}, old_fitness: {}, new fitness: {}, fitness_diff: {}, odds:{}", count, temp, old_fit , new_fit, fitness_diff, acceptance_odds);
            if acceptance_odds > rng.random_range(0.0..=1.0){
                ind = new_ind;
                old_fit = new_fit;

            }


        }
        temp = cooling_schedule(temp);
        count +=1;

   }
    ind 

}