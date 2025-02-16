use std::{collections::BTreeMap, vec};
mod plcmnt;
use num::{cast::AsPrimitive, ToPrimitive};
use plcmnt::{gcd_of_vector, hpwl, Bbox, Component, Placement};
use rand::prelude::*;
use colored::Colorize;
use plotters::{backend, prelude::*};

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
    /*
    fn pretty_print(&self) {
        let mut c = 0;
        for i in (&self.chromosone).into_iter().rev() {
            if *i != 0 {
                print!("{}", i.to_string().blue());
            } else {
                print!("{}", "\u{25A0}".green());
            }
            c += 1;
            if c >= self.x_sz {
                c = 0;
                println!("");
            }
        }
    }
    fn tuple_to_index(&self, tple: (usize, usize)) -> usize {
        (self.x_sz - tple.0) + (tple.1 * self.x_sz)
    }
    fn is_valid(&self) -> bool {
        let mut valid: bool = true;
        let mut a: BTreeMap<(usize, usize), usize> = BTreeMap::new();
        let mut count: usize = 1;

        for c in &self.comp_list {
            let mut c_space = (*c)
                .bbox
                .as_btree(self.discretization.try_into().unwrap(), count);
            count += 1usize;

            for k in c_space.iter() {
                let x = k.0 .0;
                let y = k.0 .1;
                let val = *k.1;
                let try_val = self.chromosone[y][x];
                if val != try_val {
                    return false;
                }
            }
        }
        valid
    }
    fn get_locs(&self, search_val: usize) -> Vec<(usize,usize)>{
        let ret_v = Vec::new();
        let mut y: usize = 0;
        let mut x: usize = 0;
        while y <= self.chromosone.len(){
            y += 1;
            while x <= self.chromosone[y].len(){
                if self.chromosone[y][x] == search_val{
                    self.chromosone
                }
            }

        }
    }   
    fn move_comp(&mut self, a: usize, x:usize, y:usize ){
        // lets first check to see if the new coords are in bounds
        let mut a_comp = &mut (self.comp_list[a - 1]);
        let mut new_bbox = a_comp.try_move_to(x.to_i32().unwrap(), y.to_i32().unwrap());
        let mut worked = true;
        if new_bbox.is_out_of_bounds(&Bbox::new(0, self.x_sz * disc , 0, self.y_sz * disc)){worked = false};
        let mut c_space: BTreeMap<(usize, usize), usize> = new_bbox.as_btree(self.discretization.try_into().unwrap(), a);



        
    }
    
 
    fn swap(&mut self, a: usize, b: usize) {
        let mut old_coords: BTreeMap<(usize, usize), usize> = BTreeMap::new();
        let mut new_coords: BTreeMap<(usize, usize), usize> = BTreeMap::new();

        //We need to zero, so lets grab the coords and also hold on to them
        let a_comp = &(self.comp_list[a - 1]);
        let mut c_space = (a_comp)
            .bbox
            .as_btree(self.discretization.try_into().unwrap(), 0);
        let old_a_loc = (a_comp.bbox.x1, a_comp.bbox.y1);
        old_coords.append(&mut c_space);

        let b_comp = &(self.comp_list[b - 1]);
        let mut c_space = (b_comp)
            .bbox
            .as_btree(self.discretization.try_into().unwrap(), 0);
        let old_b_loc = (b_comp.bbox.x1, b_comp.bbox.y1);
        old_coords.append(&mut c_space);

        //Now we want to swap locations
        let mut a_comp = &mut (self.comp_list[a - 1]);
        a_comp.move_to(old_b_loc.0, old_b_loc.1);
        let mut c_space = (a_comp)
            .bbox
            .as_btree(self.discretization.try_into().unwrap(), a);
        new_coords.append(&mut c_space);

        let mut b_comp = &mut (self.comp_list[b - 1]);
        b_comp.move_to(old_a_loc.0, old_a_loc.1);
        let mut c_space = (b_comp)
            .bbox
            .as_btree(self.discretization.try_into().unwrap(), b);
        new_coords.append(&mut c_space);
        for k in old_coords.iter() {
            let val = k.1;
            let idx = self.tuple_to_index(*k.0);
            self.chromosone[idx] = *val;
        }
        for k in new_coords.iter() {
            let val = k.1;
            let idx = self.tuple_to_index(*k.0);
            self.chromosone[idx] = *val;
        }
    }
*/
    fn move_to_new(&mut self, a: usize) {
        let mut rng = rand::rng();
        let x = rng.random_range(0..self.pl_area.x2);
        let y = rng.random_range(0..self.pl_area.y2);
        //We need to zero, so lets grab the coords and also hold on to them
        //let a: usize = 2;
        let mut a_comp = &mut (self.comp_list[a - 1]);
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
            let mut a_comp = &mut (self.comp_list[a - 1]);
            a_comp.move_to(old_pos.0, old_pos.1);

        }else{

            //println!("{}", "GOOD".green());
        }
        
    }

    fn score(&mut self) -> usize {
        hpwl(&mut self.comp_list)
    }

    fn rotate(&mut self, a: usize, rotation: i32) {
        let mut a_comp = &mut (self.comp_list[a - 1]);
      
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
            let mut a_comp = &mut (self.comp_list[a - 1]);
            a_comp.rotate_comp(360 - rotation);
            
            //a_comp.move_to(old_pos.0, old_pos.1);

        }else{

            //println!("{}", "GOOD".green());
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
    let mut pl = Placement {
        components: comps,
        placement_area: placement_area,
    };

    let mut id = Individual::new(pl);
    //id.rotate(2, 90);
    //id.rotate(2, 270);
    //id.rotate(3, 0);
    //id.rotate(2, 180);
    //id.pretty_print();
    let mut backend: BitMapBackend<'_> = BitMapBackend::new("0.png", (pl_width.try_into().unwrap(), pl_height.try_into().unwrap()));
    // And if we want SVG backend
    // let backend = SVGBackend::new("output.svg", (800, 600));
    //backend.draw_rect((50, 50), (200, 150), &RED, true)?;
    id.plot(&mut backend);
    backend.present();
    println!("{}", id.score());
    //id.swap(1, 3);
    id.rotate(1, 90);
    for _ in 0..1000 {
        //id.swap(1, 2);
        id.rotate(1, random_rotation());
        id.move_to_new(1);
        //id.swap(1, 2);
        id.move_to_new(2);
        id.move_to_new(3);
        //println!("{}", "++++++++++++++++++++++++".red());
        //id.pretty_print();
        //println!("{}","++++++++++++++++++++++++".red());
        //id.move_to_new(1);
    }
    println!("{}", id.score());

    /*
    //id.swap();

    println!("{}", id.score());

    let x = id.is_valid();
    if x {
        id.to_tex();
    }
    */
    //println!("{}",(c1.string()))
    let mut backend: BitMapBackend<'_> = BitMapBackend::new("1.png", (pl_width.try_into().unwrap(), pl_height.try_into().unwrap()));
    // And if we want SVG backend
    // let backend = SVGBackend::new("output.svg", (800, 600));
    //backend.draw_rect((50, 50), (200, 150), &RED, true)?;
    id.plot(&mut backend);
    backend.present()?;
    
    Ok(())
}
