mod plcmnt;
use plcmnt::{hpwl, is_valid, placement_area, Bbox, Component, Pin, Placement};
mod kicad_parse;
use colored::Colorize;
use kicad_parse::parse_file;
use plotters::coord::types::RangedCoordf64;
use plotters::prelude::*;
use rand::prelude::*;
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
    fn plot(&self, output_path: &str) {
        let padding = 10.0;
        let scale = 10.0;

        let pl_width = scale * (self.pl_area.get_width_fl() + padding * 2.0);
        let pl_height = scale * (self.pl_area.get_height_fl() + padding * 2.0);
        let style = TextStyle::from(("sans-serif", scale).into_font()).color(&RED);
        let backend = BitMapBackend::new(
            output_path,
            (pl_width.floor() as u32, pl_height.round() as u32),
        )
        .into_drawing_area();
        let backend = backend.apply_coord_spec(Cartesian2d::<RangedCoordf64, RangedCoordf64>::new(
            0f64..pl_width,
            0f64..pl_height,
            (0..pl_width.floor() as i32, 0..pl_height.round() as i32),
        ));
        backend.fill(&WHITE);
        let get_rect = |comp: &Component| {
            //let x =
            let ul: (f64, f64) = (
                (comp.bbox.x1 + padding) * scale,
                (comp.bbox.y2 + padding) * scale,
            );
            let br: (f64, f64) = (
                (comp.bbox.x2 + padding) * scale,
                (comp.bbox.y1 + padding) * scale,
            );
            //let ee: EmptyElement<(f64, f64), _> = EmptyElement::at(ul) ;
            Rectangle::new(
                [ul, br],
                ShapeStyle::from(&RGBColor(129, 133, 137)).filled(),
            )
            //+ Text::new(format!("({})",comp.refdes),(0.0, 0.0), ("sans-serif", 15.0).into_font())
        };
        let get_pin_rect = |comp: &Pin| {
            //let x =
            let ul: (f64, f64) = (
                (comp.bbox.x1 + padding) * scale,
                (comp.bbox.y2 + padding) * scale,
            );
            let br: (f64, f64) = (
                (comp.bbox.x2 + padding) * scale,
                (comp.bbox.y1 + padding) * scale,
            );
            //let ee: EmptyElement<(f64, f64), _> = EmptyElement::at(ul) ;
            if comp.net == 11 {
                Rectangle::new([ul, br], ShapeStyle::from(&GREEN).filled())
            } else {
                Rectangle::new([ul, br], ShapeStyle::from(&RED).filled())
            }
            //+ Text::new(format!("({})",comp.refdes),(0.0, 0.0), ("sans-serif", 15.0).into_font())
        };
        let label_pin = |comp: &Pin| {
            let ul: (f64, f64) = (
                (comp.bbox.x1 + padding) * scale,
                (comp.bbox.y2 + padding) * scale,
            );
            Text::new(
                format!("({})", comp.refdes),
                ul,
                ("sans-serif", 15.0).into_font(),
            )
        };
        let label_comp = |comp: &Component| {
            let ul: (f64, f64) = (
                (comp.bbox.x1 + padding) * scale,
                (comp.bbox.y2 + padding) * scale,
            );
            Text::new(
                format!("({})", comp.refdes),
                ul,
                ("sans-serif", 15.0).into_font(),
            )
        };
        //plot pcb
        let ul = (
            (self.pl_area.x1 + padding) * scale,
            (self.pl_area.y2 + padding) * scale,
        );
        let br = (
            (self.pl_area.x2 + padding) * scale,
            (self.pl_area.y1 + padding) * scale,
        );
        let ur = (
            (self.pl_area.x2 + padding) * scale,
            (self.pl_area.y2 + padding) * scale,
        );
        let bl = (
            (self.pl_area.x1 + padding) * scale,
            (self.pl_area.y1 + padding) * scale,
        );
        //let _ = backend.draw_rect(ul,br , &RGBAColor(0,255,0, 0.7), false);
        //let _ = backend.draw_text(&format!("{}, {}", self.pl_area.x2,self.pl_area.y2),&style, ur );
        //let _ = backend.draw_text(&format!("{}, {}", self.pl_area.x1, self.pl_area.y1),&style, bl );

        for i in &self.comp_list {
            let ii = get_rect(i);
            backend.draw(&label_comp(i));
            backend.draw(&ii);
            for p in &i.pins {
                let ii = get_pin_rect(p);
                //backend.draw(&ii);
            }

            /*
            let style = TextStyle::from(("sans-serif", scale).into_font()).color(&RED);
            let text_loc = ((i.bbox.x1 + padding )*scale  , (i.bbox.centery + padding) * scale );
            //let _ = backend.draw_rect(ul,br , &RGBColor(129,133,137), true);
            let _ = backend.draw_text(&i.refdes,&style, text_loc );


                let ul  =((p.bbox.x1 + padding)*scale , (p.bbox.y2 + padding)*scale);
                let br = ((p.bbox.x2+ padding)*scale, (p.bbox.y1+ padding)*scale);
                let style = TextStyle::from(("sans-serif", scale).into_font()).color(&RED);
                let text_loc = ((p.bbox.x1 + padding )*scale  , (p.bbox.centery + padding) * scale );
                if p.net != 0{

                    //let _ = backend.draw_rect(ul,br , &RGBColor(0,255,1), true);
                }
                else{
                    //let _ = backend.draw_rect(ul,br , &RGBColor(255,1,1), true);

                }
               let _ = backend.draw_text(&format!("{}.{}", &p.refdes,&p.net) ,&style, text_loc );


            }*/
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
        let mut okay = a_comp.bbox.is_out_of_bounds(&self.pl_area);

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
        let x = rng.random_range(qk_comp.get_width_fl()..self.pl_area.x2);
        let y = rng.random_range(qk_comp.get_height_fl()..self.pl_area.y2);
        //We need to zero, so lets grab the coords and also hold on to them
        //let a: usize = 2;
        self.move_comp(a, x, y);
    }

    fn score(&self) -> f64 {
        is_valid(&self.comp_list) * placement_area(&self.comp_list) * hpwl(&self.comp_list)
    }

    fn rotate(&mut self, a: usize, rotation: i32) -> bool {
        let a_comp = &mut (self.comp_list[a - 1]);

        a_comp.rotate_comp(rotation);
        let a_comp = &(self.comp_list[a - 1]);
        let mut okay = a_comp.bbox.is_out_of_bounds(&self.pl_area);

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
fn tester(){

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
    let pl_2 = pl.clone();
    let  id2 = Individual::new(pl_2);
    id2.plot("0.png");
    let gen_mult = 100;
    let test_cases: Vec<(usize, i32)> = vec![(10, 10000 * gen_mult ), (20, 5000 * gen_mult), (50,2000 * gen_mult), (100,1000 * gen_mult), (200, 500 * gen_mult), (500,200 * gen_mult)];
    for i in test_cases{

        let mut population: Vec<Individual> = Vec::new();
        let pop_size = i.0;
        for _ in 0..pop_size{
            let pl_2 = pl.clone();
            let  id2 = Individual::new(pl_2);
            population.push(id2);

        }



        // And if we want SVG backend
        // let backend = SVGBackend::new("output.svg", (800, 600));
        //backend.draw_rect((50, 50), (200, 150), &RED, true)?;
        let id = &mut population[0];
        id.plot("0.png");
        println!("{}", format!("+++++++Test (Population Size: {} , Generations {}) +++++++", pop_size, i.1).green());
        println!("Original Score: {}", id.score());
        let now = Instant::now();
        //id.swap(1, 3);
        //id.rotate(1, 90);
        for _ in 0..i.1 {
            for ind in &mut population{
                ind.mutate();
            }
            for i in (0..pop_size).step_by(2){
                let parent_a: & Individual = &population[i];
                let parent_b: & Individual = &population[i + 1];
                let child_a: Individual =  parent_a.crossover(parent_b);
                let child_b: Individual =  parent_b.crossover(parent_a);
                population.push(child_a);
                population.push(child_b);
            }
            population.sort_by(|a: &Individual, b: &Individual| {
                let a_s = a.score();
                let b_s = b.score();
                a_s.cmp(&b_s)}
            );
            population.truncate(pop_size);
        }
        /*
        */
        let id = &mut population[0];
        println!("New Score: {}", id.score());

        id.plot(&format!("test-{}x{}.png", pop_size, i.1));
        let elapsed_time = now.elapsed();
        println!("Test took {}.{} seconds.",elapsed_time.as_secs(), elapsed_time.subsec_millis());
        println!("![{}]({})",&format!("test-{}x{}.png", pop_size, i.1),&format!("test-{}x{}.png", pop_size, i.1) );
        println!("{}", "+++++++Test Over+++++++".to_string().green());
        println!();
    }


}
    */
fn main() {
    let pl = parse_file();
    let id = Individual::new(pl);
    //id.mutate();
    id.plot("tester.png");
}
