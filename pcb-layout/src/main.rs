use std::{cmp::Ordering, process::ChildStdin, vec};
mod plcmnt;
use num::ToPrimitive;
use plcmnt::{hpwl, Bbox, Component, Placement};
use rand::prelude::*;
use plotters::prelude::*;

fn random_rotation() -> i32 {
    // Get an RNG:
    let mut rng = rand::rng();
    let opts = [90, 180, 270];
    let choice = opts.choose(&mut rng).unwrap();
    *choice
}
struct Individual {
    
    comp_list: Vec<Component>,
    pl_area: Bbox
}

impl Individual {
    fn new(pl: Placement) -> Self {
        
        
        Individual {
            
            comp_list: pl.components,
            pl_area : pl.placement_area
        }
    }
    fn plot(&self, backend :&mut BitMapBackend<'_>){
        for i in &self.comp_list{
            let mut rng = rand::rng();
            //let x = ;
            backend.draw_rect((i.bbox.x1, i.bbox.y2), (i.bbox.x2, i.bbox.y1), &RGBColor(0,rng.random_range(0..255),rng.random_range(0..255)), true);
        } 


    }
    
    fn move_comp(&mut self, a: usize,x:i32, y:i32) -> bool{
        let a_comp = &mut (self.comp_list[a - 1]);
        let old_pos = (a_comp.bbox.x1, a_comp.bbox.y1);
        a_comp.move_to(x.to_i32().unwrap(), y.to_i32().unwrap());
        let a_comp = & (self.comp_list[a - 1]);
        let mut okay = a_comp.bbox.is_out_of_bounds(&self.pl_area);
        
        if okay{
            let mut count = 1;
            for i in &self.comp_list{
                
                if count != a && a_comp.bbox.does_overlap(&i.bbox){okay = false};
                count += 1;
            }
        } 
        if !okay {
            //println!("{}", "BAD".red());
            let a_comp = &mut (self.comp_list[a - 1]);
            a_comp.move_to(old_pos.0, old_pos.1);
            return false;

        }else{
            return true;
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
            }else{
                let a_comp = &mut (self.comp_list[a - 1]);
                a_comp.move_to(old_a_loc.0, old_a_loc.1);
                //drop(a_comp);
                
            }

        }else{
            
            self.move_comp(b, old_b_loc.0, old_b_loc.1);
        }
        true
    }
    fn move_to_new(&mut self, a: usize) {
        let mut rng = rand::rng();
        let qk_comp = self.comp_list[a - 1].bbox;
        let x = rng.random_range( qk_comp.get_width().try_into().unwrap() .. self.pl_area.x2);
        let y = rng.random_range(qk_comp.get_height().try_into().unwrap()..self.pl_area.y2);
        //We need to zero, so lets grab the coords and also hold on to them
        //let a: usize = 2;
        self.move_comp(a, x, y);
        
    }

    fn score(&mut self) -> usize {
        hpwl(&mut self.comp_list)
    }

    fn rotate(&mut self, a: usize, rotation: i32) -> bool {
        let a_comp = &mut (self.comp_list[a - 1]);
      
        a_comp.rotate_comp(rotation);
        let a_comp = & (self.comp_list[a - 1]);
        let mut okay = a_comp.bbox.is_out_of_bounds(&self.pl_area);
        
        if okay{
            let mut count = 1;
            for i in &self.comp_list{
                
                if count != a && a_comp.bbox.does_overlap(&i.bbox){okay = false};
                count += 1;
            }
        } 
        if !okay {
            //println!("{}", "BAD".red());
            let a_comp = &mut (self.comp_list[a - 1]);
            a_comp.rotate_comp(360 - rotation);
            return false;
            //a_comp.move_to(old_pos.0, old_pos.1);

        }else{
            return true;
            //println!("{}", "GOOD".green());
        }

       
    }
    fn refdes_to_indx(&self, rfdes:String) -> usize{
        let mut i: usize = 1;
        for comp in &self.comp_list{
            if comp.refdes == rfdes{ return  i};
            i +=1;

        }
        return 0;
        
    }
    fn crossover(&self, other : & Individual) -> Individual{
        //assert!() // add assertion to ensure they are same size
        let mut rng = rand::rng();
        let x1 = rng.random_range(0..self.pl_area.x2);
        let y1 = rng.random_range(0..self.pl_area.y2);
        let select_box: Bbox = Bbox::new( x1,  rng.random_range(x1..self.pl_area.x2), y1,  rng.random_range(y1..self.pl_area.y2));
        let mut non_selected_comps: Vec<& Component> = Vec::new();
        let mut tmp_rf: Vec<&str> = Vec::new();
        for comp in &self.comp_list{
            if !comp.bbox.does_overlap(&select_box){
                //tmp_rf.push(comp.refdes);
                for oth_comp in &self.comp_list{
                    if comp.refdes == oth_comp.refdes{

                        non_selected_comps.push(&oth_comp);
                    }
                }
            }
        }
        //println!("{:?}", non_selected_comps );
        let mut child: Individual =  Individual{
            comp_list: other.comp_list.clone(),
            pl_area: self.pl_area
        };
        for comp in non_selected_comps{
            let comp_idx = child.refdes_to_indx(comp.refdes.clone());
            let could_move = child.move_comp(comp_idx, comp.bbox.x1, comp.bbox.y1);
            if ! could_move{
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
            1   => {
                let b = rng.random_range(1..self.comp_list.len() + 1);
                self.swap(a, b);
            },
            2 =>{
                self.move_to_new(a);
            }
            3 => {
                self.rotate(a, random_rotation());
            },
            _ =>{}
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let placement_area = Bbox::new(0, 36, 0, 36);
    let boxx = Bbox::new(0, 2, 0, 4);
    let mut c1 = Component {
        refdes: "C1".to_string(),
        bbox: boxx,
        rotation: 0,
    };
    let box2 = Bbox::new(4, 8, 6, 8);
    let mut c2 = Component {
        refdes: "C2".to_string(),
        bbox: box2,
        rotation: 0,
    };
    let box3 = Bbox::new(4, 13, 0, 6);
    let mut c3 = Component {
        refdes: "C3".to_string(),
        bbox: box3,
        rotation: 0,
    };
    c1.move_comp(6, 6);
    c2.move_comp(6, 6);
    c3.move_comp(6, 6);
    //c1.move_comp( 10, 11);
    //c1.rotate_comp(90);
    let pl_width = placement_area.get_width();
    let pl_height = placement_area.get_height();
    let comps: Vec<Component> = vec![c1, c2, c3];
    let pl = Placement {
        components: comps,
        placement_area: placement_area,
    };
    let mut population: Vec<Individual> = Vec::new();
    let pop_size = 100;
    for i in 0..pop_size{
        let pl_2 = pl.clone();
        let mut id2 = Individual::new(pl_2);
        population.push(id2);

    }

    
    //let mut backend: BitMapBackend<'_> = BitMapBackend::new("0.png", (pl_width.try_into().unwrap(), pl_height.try_into().unwrap()));
    //// And if we want SVG backend
    //// let backend = SVGBackend::new("output.svg", (800, 600));
    ////backend.draw_rect((50, 50), (200, 150), &RED, true)?;
    //id.plot(&mut backend);
    //let _ = backend.present();
    //println!("{}", id.score());
    //id.swap(1, 3);
    //id.rotate(1, 90);
    for _ in 0..10 {
        for ind in &mut population{
            ind.mutate();
        }
    }
    population.sort_by(|a, b| a.score().cmp( &b.score()));
    /*
    */
    let mut id = &mut population[0];
    //let mut id2 = &mut population[1];
    //println!("{}",(c1.string()))
    //println!("{:?}", id.comp_list);
    println!("{}", id.score());
    let mut backend: BitMapBackend<'_> = BitMapBackend::new("1.png", (100,100));
    // And if we want SVG backend
    // let backend = SVGBackend::new("output.svg", (800, 600));
    //backend.draw_rect((50, 50), (200, 150), &RED, true)?;
    //let c = id.crossover(&id2);
    id.plot(&mut backend);
    backend.present()?;
    
    Ok(())
}
