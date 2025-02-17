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
    
    fn move_comp(&mut self, a: usize,x:i32, y:i32) -> bool{
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
        let mut a_comp = &mut (self.comp_list[a - 1]);
        a_comp.move_to(old_b_loc.0, old_b_loc.1);
        drop(a_comp);

        if(self.move_comp(b, old_a_loc.0, old_a_loc.1)){

            if(self.move_comp(a, old_b_loc.0, old_b_loc.1)){
                return true;
            }else{
                let mut a_comp = &mut (self.comp_list[a - 1]);
                a_comp.move_to(old_a_loc.0, old_a_loc.1);
                drop(a_comp);
                
            }

        }else{
            
            self.move_comp(b, old_b_loc.0, old_b_loc.1);
        }
        true
    }
    fn move_to_new(&mut self, a: usize) {
        let mut rng = rand::rng();
        let x = rng.random_range(0..self.pl_area.x2);
        let y = rng.random_range(0..self.pl_area.y2);
        //We need to zero, so lets grab the coords and also hold on to them
        //let a: usize = 2;
        self.move_comp(a, x, y);
        
    }

    fn score(&mut self) -> usize {
        hpwl(&mut self.comp_list)
    }

    fn rotate(&mut self, a: usize, rotation: i32) -> bool {
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
            return false;
            //a_comp.move_to(old_pos.0, old_pos.1);

        }else{
            return true;
            //println!("{}", "GOOD".green());
        }

       
    }

    fn crossover(&self, other : & Individual) -> Individual{
        //assert!() // add assertion to ensure they are same size
        
        let mut child: Individual =  Individual{
            comp_list: self.comp_list.clone(),
            pl_area: self.pl_area
        };
        child

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

    let mut pl_2 = pl.clone();

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
    let mut rng = rand::rng();
    let opts:[usize; 3] = [1,2,3];
    for _ in 0..1 {
        let a = *opts.choose(&mut rng).unwrap();
        let b = *opts.choose(&mut rng).unwrap();
        id.swap(a,b);
        id.rotate(a, random_rotation());
        id.move_to_new(a);
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
