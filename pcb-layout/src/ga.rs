use crate::plcmnt;
use colored::Colorize;
use plcmnt::{hpwl, is_valid, placement_area, Bbox, Component, Placement, mst_euclidean_length};
use plotters::coord::types::RangedCoordf64;
use plotters::prelude::*;
use rand::distr::weighted::WeightedIndex;
use rand::prelude::*;
use std::collections::BTreeMap;
use std::time::Instant;

use rayon::prelude::*;

fn random_rotation(rng: &mut ThreadRng) -> i32 {
    // Get an RNG:
    //let mut rng = rand::rng();
    let opts = [90, 180, 270];
    let choice = opts.choose(rng).unwrap();
    *choice
}
/// Helper function for drawing each net
///
fn draw_nets(pins_by_net: BTreeMap<i32, Vec<(f64, f64)>>) -> Vec<PathElement<(f64, f64)>> {
    let mut end_v: Vec<PathElement<(f64, f64)>> = Vec::new();
    for v in pins_by_net.values() {
        let mut tmp_points = v.clone();
        tmp_points.sort_by(|a, b| (a.0 + a.1).total_cmp(&(b.0 + b.1)));
        end_v.push(PathElement::new(tmp_points, RED));
    }
    end_v
}
/*
 */
#[derive(Clone, Debug)]
pub struct Individual {
    comp_list: Vec<Component>,
    pl_area: Bbox,
    fitness: f64,
    fixed_comps: Vec<u32>,
}

impl Individual {
    pub fn new(pl: Placement, fixed_comps: Vec<String>) -> Self {
        let mut i = Individual {
            comp_list: pl.components,
            pl_area: pl.placement_area,
            fitness: 0.0,
            fixed_comps: Vec::new(),
        };
        
        let fixed :Vec<u32> = fixed_comps.iter().map(|x| i.refdes_to_indx(x.to_string()) as u32).collect();
        i.fixed_comps = fixed;
        i.score();
        i
    }
    /// Our Plotting Function
    /// Uses the net map to plot the GND pins as green
    pub fn plot(&self, output_path: &str, net_map: &BTreeMap<i32, String>) {
        let padding = 10.0;
        let scale = 10.0;

        let pl_width = scale * (self.pl_area.get_width_fl() + padding * 2.0);
        let pl_height = scale * (self.pl_area.get_height_fl() + padding * 2.0);
        //let style = TextStyle::from(("sans-serif", scale).into_font()).color(&RED);
        let mut pins_by_net: BTreeMap<i32, Vec<(f64, f64)>> = BTreeMap::new();
        for k in net_map.keys() {
            pins_by_net.insert(*k, Vec::new());
        }
        let backend = BitMapBackend::new(
            output_path,
            (pl_width.floor() as u32, pl_height.round() as u32),
        )
        .into_drawing_area();
        let backend = backend.apply_coord_spec(Cartesian2d::<RangedCoordf64, RangedCoordf64>::new(
            0f64..self.pl_area.get_width_fl(),
            0f64..self.pl_area.get_height_fl(),
            (0..pl_width.floor() as i32, 0..pl_height.round() as i32),
        ));
        let _ = backend.fill(&WHITE);

        //plot pcb
        let _ = backend.draw(&self.pl_area.plot(&RGBColor(22, 77, 2)));

        for i in &self.comp_list {
            //let _ =  backend.draw(&label_comp(i));
            let _ = backend.draw(&i.bbox.plot(&RGBColor(129, 133, 137)));
            for p in &i.pins {
                let net_name = net_map.get(&p.net).unwrap();

                if net_name.to_lowercase() == "gnd" {
                    let _ = backend.draw(&p.bbox.plot(&GREEN));
                } else {
                    let _ = backend.draw(&p.bbox.plot(&RED));
                    let my_vec = pins_by_net.get_mut(&p.net);
                    if let Some(vec) = my_vec {
                        vec.push((p.bbox.centerx, p.bbox.centery));
                    }
                }
                //backend.draw(&label_pin(p));
            }
        }
        let net_paths = draw_nets(pins_by_net);
        //println!("{}",net_paths.len());
        for path in net_paths {
            let _ = backend.draw(&path);
        }
        let _ = backend.present();
        /*
         */
    }
    /// Moves a comp, returns whether or not it could move it
    fn move_comp(&mut self, a: usize, x: f64, y: f64) -> bool {
        if self.fixed_comps.contains(&(a as u32)){
            return false;
        }
        let a_comp = &mut (self.comp_list[a - 1]);
        let old_pos = (a_comp.bbox.x1, a_comp.bbox.y1);
        a_comp.move_to(x, y);
        let a_comp = &(self.comp_list[a - 1]);
        let mut okay = !a_comp.bbox.is_out_of_bounds(&self.pl_area);

        if okay {
            let mut count = 1;
            for i in &self.comp_list {
                if count != a && a_comp.bbox.does_overlap(&i.bbox) {
                    okay = false
                };
                count += 1;
            }
        }
        if !okay {
            //println!("{}", "BAD".red());
            let a_comp = &mut (self.comp_list[a - 1]);
            a_comp.move_to(old_pos.0, old_pos.1);
            false
        } else {
            true
            //println!("{}", "GOOD".green());
        }
    }
    ///Swaps a comp returns if it was successful
    ///
    fn swap(&mut self, a: usize, b: usize, _rng: &mut ThreadRng) -> bool {
        //We need to zero, so lets grab the coords and also hold on to them
        let a_comp = &(self.comp_list[a - 1]);
        let old_a_loc = (a_comp.bbox.x1, a_comp.bbox.y1);

        let b_comp = &(self.comp_list[b - 1]);
        let old_b_loc = (b_comp.bbox.x1, b_comp.bbox.y1);
        let a_comp = &mut (self.comp_list[a - 1]);
        a_comp.move_to(old_b_loc.0, old_b_loc.1);
        //drop(a_comp);

        if self.move_comp(b, old_a_loc.0, old_a_loc.1) {
            if self.move_comp(a, old_b_loc.0, old_b_loc.1) {
                return true;
            } else {
                let a_comp = &mut (self.comp_list[a - 1]);
                a_comp.move_to(old_a_loc.0, old_a_loc.1);
                //drop(a_comp);
            }
        } else {
            self.move_comp(b, old_b_loc.0, old_b_loc.1);
            let a_comp = &mut (self.comp_list[a - 1]);
            a_comp.move_to(old_a_loc.0, old_a_loc.1);
        }
        true
    }
    /// Moves to a new (random) position
    fn move_to_new(&mut self, a: usize, rng: &mut ThreadRng) -> bool {
        //let mut rng = rand::rng();
        //let qk_comp = self.comp_list[a - 1].bbox;
        let x = rng.random_range(self.pl_area.x1..self.pl_area.x2);
        let y = rng.random_range(self.pl_area.y1..self.pl_area.y2);
        //We need to zero, so lets grab the coords and also hold on to them
        //let a: usize = 2;
        let mved = self.move_comp(a, x, y);
        let debug = false;
        if debug && mved {
            println!("{:?} : new points{},{}", self.pl_area, x, y);
        }
        mved
    }
    /// Our simple scoring function
    /// Is a sum of placement area + hpwl + and no overlaps
    pub fn score(&mut self) -> f64 {
        self.fitness =
            //placement_area(&self.comp_list) + 
            is_valid(&self.comp_list) + mst_euclidean_length(&self.comp_list);
        self.fitness
    }
    /// Rotates a component ```a``` x degrees
    fn rotate(&mut self, a: usize, rotation: i32) -> bool {
        let a_comp = &mut (self.comp_list[a - 1]);

        a_comp.rotate_comp(rotation);
        let a_comp = &(self.comp_list[a - 1]);
        let mut okay = !a_comp.bbox.is_out_of_bounds(&self.pl_area);

        if okay {
            let mut count = 1;
            for i in &self.comp_list {
                if count != a && a_comp.bbox.does_overlap(&i.bbox) {
                    okay = false
                };
                count += 1;
            }
        }
        if !okay {
            let a_comp = &mut (self.comp_list[a - 1]);
            a_comp.rotate_comp(-rotation);
            false
            //a_comp.move_to(old_pos.0, old_pos.1);
        } else {
            true
            //println!("{}", "GOOD".green());
        }
    }
    /// Given a string, returns the index of that refdes or 0 if it cannot be found
    fn refdes_to_indx(&self, rfdes: String) -> usize {
        let mut i: usize = 1;
        for comp in &self.comp_list {
            if comp.refdes == rfdes {
                return i;
            };
            i += 1;
        }
        0
    }
    ///Our crossover function
    /// partions an area to inherit from self and then tries to take the remaining components from other
    fn crossover(&self, other: &Individual, rng: &mut ThreadRng) -> Individual {
        //assert!() // add assertion to ensure they are same size
        //let mut rng = rand::rng();
        let x1 = rng.random_range(0.0..self.pl_area.x2);
        let y1 = rng.random_range(0.0..self.pl_area.y2);
        let select_box: Bbox = Bbox::new(
            x1,
            rng.random_range(x1..self.pl_area.x2),
            y1,
            rng.random_range(y1..self.pl_area.y2),
        );
        let mut non_selected_comps: Vec<&Component> = Vec::new();
        //let mut tmp_rf: Vec<&str> = Vec::new();
        for comp in &self.comp_list {
            if !comp.bbox.does_overlap(&select_box) {
                //tmp_rf.push(comp.refdes);
                for oth_comp in &self.comp_list {
                    if comp.refdes == oth_comp.refdes {
                        non_selected_comps.push(oth_comp);
                    }
                }
            }
        }
        //println!("{:?}", non_selected_comps );
        let mut child: Individual = Individual {
            comp_list: other.comp_list.clone(),
            pl_area: self.pl_area,
            fitness: self.fitness,
            fixed_comps: self.fixed_comps.clone(),
        };

        for comp in non_selected_comps {
            let comp_idx = child.refdes_to_indx(comp.refdes.clone());
            let could_move = child.move_comp(comp_idx, comp.bbox.x1, comp.bbox.y1);
            if !could_move {
                //TODO this could result in overlap
                //could_move =
                child.move_to_new(comp_idx, rng);
            }
        }
        child.score();
        child
    }
    /// Total mutation function
    pub fn mutate(&mut self, rng: &mut ThreadRng) -> bool {
        //let mut rng = rand::rng();
        let pickable_comps: Vec<usize> = (1..self.comp_list.len() +1).filter(|i| !self.fixed_comps.contains(&(*i as u32))).collect();
        let a = *pickable_comps.choose(rng).unwrap();
        let c = rng.random_range(1..4);
        //TODO should add test 
        //let c = ;
        match c {
            1 => {
                let b =  *pickable_comps.choose(rng).unwrap();
                self.swap(a, b, rng)
            }
            2 => self.move_to_new(a, rng),
            3 => self.rotate(a, random_rotation(rng)),
            _ => false,
        }
    }
}
/// Little helper fn for generating 100 images
///
///
/// I wanted to use the image crate to turn them into a gif but for right now I am just using gimp
pub fn generate_animation(pl: Placement) -> Vec<String> {
    let mut rng = rand::rng();
    let mut population: Vec<Individual> = Vec::new();
    let mut scores: Vec<f64> = Vec::new();
    let frame_count = 100;
    let mut file_names = Vec::new();
    let pop_size = 100;
    for _ in 0..pop_size {
        let pl_2 = pl.clone();
        let id2 = Individual::new(pl_2, vec![]);
        population.push(id2);
    }
    for count in 0..frame_count {
        for ind in &mut population {
            ind.mutate(&mut rng);
            ind.score();
        }
        //elitist_selection(&mut population);
        ev_selection(&mut population);
        scores.push(population[0].fitness);
        let id = &mut population[0];
        let fname = format!("anim//{}.png", count);
        id.plot(&fname, &pl.net_map);
        file_names.push(fname);
    }
    file_names
}
/// Simple selector using  monte-carlo
/// Note: we select and crossover in the same function because we are lazy
///
pub fn ev_selection(population: &mut Vec<Individual>) {
    let weights: Vec<f64> = population.iter().map(|i| 1.0 / i.fitness).collect();
    let dist = WeightedIndex::new(&weights).unwrap();
    let pop_size = population.len();
    //let new_vec = Vec::new();
    let mut rng = rand::rng();
    for _ in 0..pop_size {
        let parent_a: &Individual = &population[dist.sample(&mut rng)];
        let parent_b: &Individual = &population[dist.sample(&mut rng)];
        //Its important to note after crossover the children *should* be scored
        let child_a: Individual = parent_a.crossover(parent_b, &mut rng);
        let child_b: Individual = parent_b.crossover(parent_a, &mut rng);
        population.push(child_a);
        population.push(child_b);
    }
    population.reverse();
    population.truncate(pop_size);
}
/// Simple Selector using an "elitist selection" as described by the paper
/// Note: we select and crossover in the same function because we are lazy
///
pub fn elitist_selection(population: &mut Vec<Individual>) {
    let weights: Vec<f64> = population.iter().map(|i| 1.0 / i.fitness).collect();
    let dist = WeightedIndex::new(&weights).unwrap();
    let pop_size = population.len();
    let mut rng = rand::rng();

    for _ in (0..pop_size).step_by(2) {
        let parent_a: &Individual = &population[dist.sample(&mut rng)];
        let parent_b: &Individual = &population[dist.sample(&mut rng)];
        let child_a: Individual = parent_a.crossover(parent_b, &mut rng);
        let child_b: Individual = parent_b.crossover(parent_a, &mut rng);

        population.push(child_a);
        population.push(child_b);
    }
    population.sort_by(|a: &Individual, b: &Individual| {
        let a_s = a.fitness;
        let b_s = b.fitness;

        a_s.partial_cmp(&b_s).unwrap()
    });
    population.truncate(pop_size);
}
/// The actual runner for our GA
/// takes an initial placement, the size of each population, the number of generations it can run,
/// should it output and a closure representing the selection/crossover operator. (and now the number of "threads")
/// Reccomended values I don't know
/// It should return a vec of scores so you can plot performance if you want
pub fn genetic_algorithim(
    mut pl: Placement,
    pop_size: u32,
    num_generations: u32,
    output: bool,
    selection_algo: fn(&mut Vec<Individual>) -> (),
    fixed_refdes: Vec<String>,
    nthreads: u32,
) -> Vec<f64> {
    let (offsetx,offsety) = pl.shift_placement(0.0, 0.0);
    let mut population: Vec<Individual> = Vec::new();
    //let mut children = vec![];
    let scores: Vec<f64> = Vec::new();
    for _ in 0..pop_size {
        let pl_2 = pl.clone();
        let mut id2 = Individual::new(pl_2, fixed_refdes.clone());
        id2.fixed_comps = vec![3];
        population.push(id2);
    }
    let id = &mut population[0];
    if output {
        println!(
            "{}",
            format!(
                "+++++++Test (Population Size: {} , Generations {}) +++++++",
                pop_size, num_generations
            )
            .green()
        );
        println!("Original Score: {}", id.score());
    }

    let now = Instant::now();

    // Clone the original population into n separate populations
    let num_populations = 3;
    //lets only clone/migrate every x generations
    let reset_num = 20;
    //Double par seems like it only add overhead sadly
    let use_double_par = false;
    let mut populations: Vec<&mut [Individual]> = population
        .chunks_mut((pop_size / nthreads).try_into().unwrap())
        .collect();
    for cur_generation in 1..num_generations / num_populations {
        // Make our new populations

        if cur_generation % reset_num == 1 {
            //populations  = (0..num_populations).map(|_|  population.clone()).collect();
        }

        // Apply evolution in parallel to all populations
        //NOTE: THIS STARTED AS CHAT GPT CODE. IT DOESN'T REALLY RESEMBLE IT ANY MORE
        populations.par_iter_mut().for_each(|pop| {
            let mut rng = rand::rng();
            if use_double_par {
                // This cant make this better
                pop.par_iter_mut().for_each(|ind| {
                    let mut rng = rand::rng();
                    if ind.mutate(&mut rng) {
                        ind.score();
                    }
                });
            } else {
                for ind in pop.iter_mut() {
                    if ind.mutate(&mut rng) {
                        ind.score();
                    }
                }
            }

            selection_algo(&mut pop.to_vec());
        });
        //CLONE IS EXPENSIVE?

        if cur_generation % reset_num == 0 {
            selection_algo(&mut population);
            populations = population
                .chunks_mut((pop_size / nthreads).try_into().unwrap())
                .collect();
        }
    }
    //Lets make sure we are in order
    population.sort_by(|a: &Individual, b: &Individual| {
        let a_s = a.fitness;
        let b_s = b.fitness;

        a_s.partial_cmp(&b_s).unwrap()
    });

    let id = &mut population[0];
    if output {
        println!("New Score: {}", id.score());

        let elapsed_time = now.elapsed();
        println!(
            "\nTest took {}.{} seconds.",
            elapsed_time.as_secs(),
            elapsed_time.subsec_millis()
        );
        println!("\n{}", "+++++++Test Over+++++++".to_string().green());
        id.plot(
            &format!("tests/test-{}x{}.png", pop_size, num_generations),
            &pl.net_map,
        );
        let mut pl2 = pl.clone();
        pl2.components = id.comp_list.clone();
        pl2.shift_placement(-1.0 * offsetx, -1.0 * offsety);
        pl2.to_csv("output.csv".to_string());
    }

    scores
}
