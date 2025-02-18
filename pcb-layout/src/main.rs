use std::{cmp::Ordering, process::ChildStdin, vec};
mod plcmnt;
use num::ToPrimitive;
use plcmnt::{hpwl, is_valid, Bbox, Component, Placement};
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
    fn plot(&self, output_path: &str){
        let padding: i32 = 10;
        let scale: i32 = 12;
        let pl_width:i32 = scale * (self.pl_area.get_width().to_i32().unwrap() + padding * 2);
        let pl_height:i32 = scale * (self.pl_area.get_height().to_i32().unwrap() + padding * 2);
        let style = TextStyle::from(("sans-serif", scale).into_font()).color(&RED);
        let mut backend: BitMapBackend<'_> = BitMapBackend::new(output_path, (pl_width.try_into().unwrap() , pl_height.try_into().unwrap()));
        //plot pcb
        let ul  =((self.pl_area.x1 + padding)*scale , (self.pl_area.y2 + padding)*scale);
        let br = ((self.pl_area.x2+ padding)*scale, (self.pl_area.y1+ padding)*scale);
        let ur  =((self.pl_area.x2 + padding)*scale , (self.pl_area.y2 + padding)*scale);
        let bl = ((self.pl_area.x1+ padding)*scale, (self.pl_area.y1+ padding)*scale);
        backend.draw_rect(ul,br , &RGBAColor(0,255,0, 0.7), false);
        backend.draw_text(&format!("{}, {}", self.pl_area.x2,self.pl_area.y2),&style, ur );
        backend.draw_text(&format!("{}, {}", self.pl_area.x1, self.pl_area.y1),&style, bl );

        let mut rgb = 0;
        for i in &self.comp_list{
            let mut rng = rand::rng();
            rgb += 30;
            rgb %= 254;
            //let x = ;
            let ul  =((i.bbox.x1 + padding)*scale , (i.bbox.y2 + padding)*scale);
            let br = ((i.bbox.x2+ padding)*scale, (i.bbox.y1+ padding)*scale);
            let style = TextStyle::from(("sans-serif", scale).into_font()).color(&RED);
            let text_loc = ((i.bbox.x1 + padding )*scale  , (i.bbox.centery + padding) * scale );
            backend.draw_rect(ul,br , &RGBColor(rgb,rgb,128), true);
            backend.draw_text(&i.refdes,&style, text_loc );
        } 
        let _ = backend.present();

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

    fn score(& self) -> usize {
        is_valid(& self.comp_list ) * hpwl(& self.comp_list)
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
            let a_comp = &mut (self.comp_list[a - 1]);
            a_comp.rotate_comp( -1* rotation);
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
        //let mut tmp_rf: Vec<&str> = Vec::new();
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
    let box2 = Bbox::new(32, 36, 34, 36);
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
    let mut c4 = c2.clone();
    c4.refdes = "C4".to_string();
    let mut c5 = c2.clone();
    c5.refdes = "C5".to_string();
    c1.move_comp(6, 6);
    //c2.move_comp(6, 6);
    c4.move_comp(0, -6);
    c5.move_comp(15, 2);
    c3.move_comp(6, 6);
    //c1.move_comp( 10, 11);
    //for i in 1..2000{ c1.rotate_comp(90);};
    let pl_width = placement_area.get_width();
    let pl_height = placement_area.get_height();
    let comps: Vec<Component> = vec![c1,c2,c3,c4,c5];
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

    
    
    // And if we want SVG backend
    // let backend = SVGBackend::new("output.svg", (800, 600));
    //backend.draw_rect((50, 50), (200, 150), &RED, true)?;
    let mut id = &mut population[0];
    id.plot("0.png");
    
    //println!("{}", id.score());
    //id.swap(1, 3);
    //id.rotate(1, 90);
    for _ in 0..10000 {
        for ind in &mut population{
            ind.mutate();
        }
        for i in (0..pop_size).step_by(2){
            let parent_a: & Individual = &population[i];
            let parent_b: & Individual = &population[i + 1];
            let mut child_a: Individual =  parent_a.crossover(parent_b);
            let mut child_b: Individual =  parent_b.crossover(parent_a);
            population.push(child_a);
            population.push(child_b);
        }
        population.sort_by(|a: &Individual, b: &Individual| { 
            let a_s = (a.score()).clone();
            let b_s = (b.score()).clone();
            a_s.cmp(&b_s)}
        );
        population.truncate(pop_size);
    }
    /*
    */
    let mut id = &mut population[0];
    //let mut id2 = &mut population[1];
    //println!("{}",(c1.string()))
    println!("{:?}", id.comp_list);
    println!("{}", id.score());
    println!("{}", is_valid(&id.comp_list));
    
    id.plot("1.png");
    
    Ok(())
}
