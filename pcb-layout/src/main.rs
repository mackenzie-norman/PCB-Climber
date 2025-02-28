mod plcmnt;
use plcmnt::{hpwl, is_valid, placement_area, Bbox, Component, Placement};
mod kicad_parse;
use clap::Parser;
use colored::Colorize;
use kicad_parse::parse_file;
use plotters::coord::types::RangedCoordf64;
use plotters::prelude::*;
use rand::prelude::*;
use std::collections::BTreeMap;
use std::time::Instant;

fn random_rotation() -> i32 {
    // Get an RNG:
    let mut rng = rand::rng();
    let opts = [90, 180, 270];
    let choice = opts.choose(&mut rng).unwrap();
    *choice
}
struct Individual {
    comp_list: Vec<Component>,
    pl_area: Bbox,
}

impl Individual {
    fn new(pl: Placement) -> Self {
        Individual {
            comp_list: pl.components,
            pl_area: pl.placement_area,
        }
    }
    fn plot(&self, output_path: &str, net_map: &BTreeMap<i32, String>) {
        let padding = 10.0;
        let scale = 10.0;

        let pl_width = scale * (self.pl_area.get_width_fl() + padding * 2.0);
        let pl_height = scale * (self.pl_area.get_height_fl() + padding * 2.0);
        //let style = TextStyle::from(("sans-serif", scale).into_font()).color(&RED);
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
                }
                //backend.draw(&label_pin(p));
            }
        }
        let _ = backend.present();
        /*
         */
    }

    fn move_comp(&mut self, a: usize, x: f64, y: f64) -> bool {
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
    fn swap(&mut self, a: usize, b: usize) -> bool {
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
        }
        true
    }
    fn move_to_new(&mut self, a: usize) {
        let mut rng = rand::rng();
        let qk_comp = self.comp_list[a - 1].bbox;
        let x = rng.random_range(self.pl_area.x1..self.pl_area.x2);
        let y = rng.random_range(self.pl_area.y1..self.pl_area.y2);
        //We need to zero, so lets grab the coords and also hold on to them
        //let a: usize = 2;
        let mved = self.move_comp(a, x, y);
        let debug = false;
        if debug && mved {
            println!("{:?} : new points{},{}", self.pl_area, x, y);
        }
    }

    fn score(&self) -> f64 {
        is_valid(&self.comp_list) * placement_area(&self.comp_list) * hpwl(&self.comp_list)
    }

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
    fn crossover(&self, other: &Individual) -> Individual {
        //assert!() // add assertion to ensure they are same size
        let mut rng = rand::rng();
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
        };
        for comp in non_selected_comps {
            let comp_idx = child.refdes_to_indx(comp.refdes.clone());
            let could_move = child.move_comp(comp_idx, comp.bbox.x1, comp.bbox.y1);
            if !could_move {
                child.move_to_new(comp_idx);
            }
        }
        child
    }

    fn mutate(&mut self) {
        let mut rng = rand::rng();
        let a = rng.random_range(1..self.comp_list.len() + 1);
        let c = rng.random_range(1..4);
        //let c = 2;
        match c {
            1 => {
                let b = rng.random_range(1..self.comp_list.len() + 1);
                self.swap(a, b);
            }
            2 => {
                self.move_to_new(a);
            }
            3 => {
                self.rotate(a, random_rotation());
            }
            _ => {}
        }
    }
}
/*
fn synth_pl() {

    let placement_area = Bbox::new(0, 36, 0, 36);
    let pin_boxx = Bbox::new(0, 2, 0,1);
    let base_pin = Pin{refdes :"C1".to_string(), net: 0, bbox:pin_boxx };
    let mut base_pin_2 = Pin{refdes :"C1".to_string(), net: 1, bbox:pin_boxx };
    base_pin_2.move_pin(0, 3);
    let boxx = Bbox::new(0, 2, 0, 4);
    let mut c1 = Component {
        refdes: "C1".to_string(),
        bbox: boxx,
        rotation: 0,
        pins : vec![base_pin.clone(), base_pin_2.clone()]
    };
    let mut b_pin = base_pin.clone();
    let mut b_pin2 = base_pin_2.clone();
    b_pin.refdes = "C2".to_string();
    b_pin2.refdes = "C2".to_string();
    b_pin.move_pin(34, 32);
    b_pin2.move_pin(34, 32);
    b_pin2.net = 2;
    let box2 = Bbox::new(34, 36, 32, 36);
    let c2 = Component {
        refdes: "C2".to_string(),
        bbox: box2,
        rotation: 0,
        pins : vec![b_pin, b_pin2]
    };
    let mut base_pin = Pin{refdes :"C3".to_string(), net: 0, bbox:pin_boxx };
    base_pin.move_pin(11, 5);
    let mut base_pin_2 = Pin{refdes :"C3".to_string(), net: 1, bbox:pin_boxx };
    base_pin_2.move_pin(7, 5);
    let mut base_pin_3 = Pin{refdes :"C3".to_string(), net: 2, bbox:pin_boxx };
    base_pin_3.move_pin(4, 0);
    let box3 = Bbox::new(4, 13, 0, 6);
    let mut c3 = Component {
        refdes: "C3".to_string(),
        bbox: box3,
        rotation: 0,
        pins : vec![base_pin.clone(), base_pin_2.clone(), base_pin_3]
    };
    let mut c4 = c2.clone();
    c4.set_refdes("C4".to_string());
    let mut c5 = c1.clone();
    c5.set_refdes("C5".to_string());
    c1.move_comp(6, 6);
    //c2.move_comp(6, 6);
    c4.move_comp(0, -6);
    c5.move_comp(15, 2);
    c3.move_comp(6, 6);
    //c1.move_comp( 10, 11);
    //for i in 1..2000{ c1.rotate_comp(90);};
    c3.rotate_comp(180);
    //c3.rotate_comp(90);
    let comps: Vec<Component> = vec![c1, c2,c3,c4, c5];

    let pl = Placement {
        components: comps,
        placement_area,
    };
}
*/
fn generate_animation(pl: Placement) -> Vec<String> {
    let pl_2 = pl.clone();
    let mut id2 = Individual::new(pl_2);
    //id2.plot("anim//0.png", &pl.net_map);
    let frame_count = 100;
    let mut file_names = Vec::new();
    for count in 0..frame_count {
        let fname = format!("anim//{}.png", count);
        id2.plot(&fname, &pl.net_map);
        id2.mutate();
        file_names.push(fname);
    }
    file_names
}
fn genetic_algorithim(pl: Placement, pop_size: u32, num_generations: u32, output: bool) -> Vec<f64> {
    let mut population: Vec<Individual> = Vec::new();
    let mut scores: Vec<f64> = Vec::new();
    for _ in 0..pop_size {
        let pl_2 = pl.clone();
        let id2 = Individual::new(pl_2);
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
    for _ in 0..num_generations {
        for ind in &mut population {
            ind.mutate();
        }
        for i in (0..pop_size).step_by(2) {
            let parent_a: &Individual = &population[i as usize];
            let parent_b: &Individual = &population[(i + 1) as usize];
            let child_a: Individual = parent_a.crossover(parent_b);
            let child_b: Individual = parent_b.crossover(parent_a);
            population.push(child_a);
            population.push(child_b);
        }
        population.sort_by(|a: &Individual, b: &Individual| {
            let a_s = a.score();
            let b_s = b.score();

            a_s.partial_cmp(&b_s).unwrap()
        });
        population.truncate(pop_size as usize);
        scores.push(population[0].score());
    }
    /*
     */
    let id = &mut population[0];
    if output {
        println!("New Score: {}", id.score());

        id.plot(
            &format!("test-{}x{}.png", pop_size, num_generations),
            &pl.net_map,
        );
        let elapsed_time = now.elapsed();
        println!(
            "\nTest took {}.{} seconds.",
            elapsed_time.as_secs(),
            elapsed_time.subsec_millis()
        );
        println!("\n{}", "+++++++Test Over+++++++".to_string().green());
    }

    scores
}
fn tester(pl: Placement) {
    let pl_2 = pl.clone();
    let id2 = Individual::new(pl_2);
    id2.plot("0.png", &pl.net_map);
    let gen_mult = 1;
    let test_cases: Vec<(u32, u32)> = vec![
        (10, 10000 * gen_mult),
        (20, 5000 * gen_mult),
        (50, 2000 * gen_mult),
        (100, 1000 * gen_mult),
        (200, 500 * gen_mult),
        (500, 200 * gen_mult),
    ];
    for i in test_cases {
        genetic_algorithim(pl.clone(), i.0, i.1, true);
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the .kicad_pcb file to use
    #[arg(short, long, default_value_t = ("../arduino_kicad/arduino UNO.kicad_pcb".to_string()) )]
    file: String,

    /// Number of generations
    #[arg(short, long, default_value_t = 1000)]
    generations: u32,
    /// How many individuals are in our popuation
    #[arg(short, long, default_value_t = 100)]
    population_size: u32,
    ///Run the testing function on our file (will overwrite gen/pop)
    #[arg(short, long, default_value_t = false)]
    test: bool,
}
fn main() {
    let args = Args::parse();
    let mut pl2: Placement = parse_file(&args.file);
    pl2.shift_placement(0.0, 0.0);

    let test = args.test;
    if test {
        tester(pl2);
    } else {
        let scores =genetic_algorithim(pl2, args.population_size, args.generations, true);
        println!("{:?}", scores);
    }
}
