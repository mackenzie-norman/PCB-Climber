use std::{collections::{btree_map, BTreeMap}, vec};
use num::{cast::AsPrimitive, integer::gcd, ToPrimitive};
mod plcmnt;
use plcmnt::{Component, Bbox, Placement};
use rand::prelude::*;


//CHAT GPT CODE JUST TO TEST
fn gcd_of_vector(nums: &[usize]) -> usize {

    let mut result = nums[0]; // Initialize with the first element

    for num in nums.iter().skip(1) {

        result = gcd(result, *num); // Calculate LCM for each pair

    }

    result

}
fn random_rotation() -> i32{
    // Get an RNG:
    let mut rng = rand::rng();
    let opts = [90,180,270];
    let choice = opts.choose(&mut rng).unwrap();
    *choice
    
}
struct Individual{
    chromosone :  Vec<Vec<usize>>,
    comp_list: Vec<Component>,
    discretization: usize,
}
impl Individual{
    fn new( pl: Placement) -> Self{
        //For now lets just say its a 6 x 6
        let mut sizes = Vec::new();
        for a in &pl.components{
            sizes.push(a.get_height());
            sizes.push(a.get_width());
        }
        //println!("{:?}", sizes);
        let disc = gcd_of_vector(&sizes);

        let mut a:BTreeMap<(usize, usize), usize > = BTreeMap::new();
        let mut count: usize = 1;
        for c in &pl.components{
            let mut c_space = c.bbox.as_btree(disc.try_into().unwrap(), count);
            a.append(&mut c_space);
            count += 1usize;
        }
        
        let mut csone_vec = Vec::new();
        let y_end: usize = (pl.placement_area.y2/disc.to_i32().unwrap() ).try_into().unwrap();
        let x_end: usize = (pl.placement_area.x2/disc.to_i32().unwrap()) .try_into().unwrap();

        for y in 0..y_end{
            let mut t_v = Vec::new();
            for x in 0..x_end{
                let coords : (usize,usize) = (x,y);
                match a.get(&coords) {
                    Some(val) =>  t_v.push(*val),
                    None => t_v.push(0)
                }
            }
            csone_vec.push(t_v)
        }
        Individual{chromosone: csone_vec, comp_list: pl.components, discretization: disc}
    }
    fn to_tex(&self) {
       for i in (&self.chromosone).into_iter().rev(){
        println!("{:?}", i);
       } 
    }

    fn is_valid(&self) -> bool{
        let mut valid: bool = true;
        let mut a:BTreeMap<(usize, usize), usize > = BTreeMap::new();
        let mut count: usize = 1;
        
        for c in &self.comp_list{
            let mut c_space = (*c).bbox.as_btree(self.discretization.try_into().unwrap(), count);
            count += 1usize;
        
            for k in c_space.iter(){
                let x = k.0.0 ;
                let y = k.0.1 ;
                let val = *k.1;
                let try_val = self.chromosone[y][x];
                println!("{val} = {try_val}");
                if val != try_val{
                    return false;
                }
            }
        }
        valid 
    }
    /* 
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
    */
    fn swap(&mut self, a:usize, b:usize) {
        let mut old_coords:BTreeMap<(usize, usize), usize > = BTreeMap::new();
        let mut new_coords:BTreeMap<(usize, usize), usize > = BTreeMap::new();

        //We need to zero, so lets grab the coords and also hold on to them 
        let a_comp = &(self.comp_list[a-1]);
        let mut c_space = (a_comp).bbox.as_btree(self.discretization.try_into().unwrap(), 0);
        let old_a_loc = (a_comp.bbox.x1, a_comp.bbox.y1);
        old_coords.append(&mut c_space);
        
        let  b_comp = &(self.comp_list[b-1]);
        let mut c_space = (b_comp).bbox.as_btree(self.discretization.try_into().unwrap(), 0);
        let old_b_loc = (b_comp.bbox.x1, b_comp.bbox.y1);
        old_coords.append(&mut c_space);
        
        //Now we want to swap locations
        let mut a_comp = &mut (self.comp_list[a-1]);
        a_comp.move_to(old_b_loc.0, old_b_loc.1);
        let mut c_space = (a_comp).bbox.as_btree(self.discretization.try_into().unwrap(), a);
        new_coords.append(&mut c_space);


        let mut  b_comp = &mut (self.comp_list[b-1]);        
        b_comp.move_to(old_a_loc.0, old_a_loc.1);
        let mut c_space = (b_comp).bbox.as_btree(self.discretization.try_into().unwrap(), b);
        new_coords.append(&mut c_space);
        for k in old_coords.iter(){
            let x = k.0.0 ;
            let y = k.0.1 ;
            let val = k.1;
            self.chromosone[y][x] = *val;
        }
        for k in new_coords.iter(){
            let x = k.0.0 ;
            let y = k.0.1 ;
            let val = k.1;
            self.chromosone[y][x] = *val;
        }
        
        



    
    }
    fn rotate(& mut self, a:usize, rotation:i32){
        let mut old_coords:BTreeMap<(usize, usize), usize > = BTreeMap::new();
        let mut new_coords:BTreeMap<(usize, usize), usize > = BTreeMap::new();

        //We need to zero, so lets grab the coords and also hold on to them 
        //let a: usize = 2;
        
        let mut a_comp = &mut (self.comp_list[a-1]);
        let mut c_space = (a_comp).bbox.as_btree(self.discretization.try_into().unwrap(), 0);
        old_coords.append(&mut c_space);
        println!("{:?}", a_comp);
        a_comp.rotate_comp(rotation);
        println!("{:?}", a_comp);

        let mut c_space = (a_comp).bbox.as_btree(self.discretization.try_into().unwrap(), a);
        new_coords.append(&mut c_space);
        for k in old_coords.iter(){
            let x = k.0.0 ;
            let y = k.0.1 ;
            let val = k.1;
            self.chromosone[y][x] = *val;
        }
        for k in new_coords.iter(){
            let x = k.0.0 ;
            let y = k.0.1 ;
            let val = k.1;
            println!("{x}{y}");
            self.chromosone[y][x] = *val;
        }

    }
}


fn main() {
    let placement_area = Bbox::new(0, 24, 0, 24);
    let  boxx= Bbox::new(0,2,0,4);
    let  mut c1 = Component{refdes: "C1".to_string(), bbox:boxx, rotation:0};
    let  box2= Bbox::new(4,8,6,8);
    let  mut c2 = Component{refdes: "C2".to_string(), bbox:box2, rotation:0};
    let   box3= Bbox::new(4,12,0,6);
    let  mut c3 = Component{refdes: "C3".to_string(), bbox:box3, rotation:0};
    c1.move_comp(6, 6);
    c2.move_comp(6, 6);
    c3.move_comp(6, 6);
    //c1.move_comp( 10, 11);
    //c1.rotate_comp(90);
    let  comps:Vec<Component> = vec![c1,c2,c3];
    let pl = Placement{components: comps, placement_area: placement_area };
    let mut id = Individual::new(pl);
    //id.to_tex();
    //id.swap();
    id.rotate(2, 90);
    id.rotate(2, 90);
    id.rotate(2, 270);
    id.rotate(3, 0);
    //id.swap(1, 3);
    //id.swap(1, 2);
    
    
    let x = id.is_valid() ;
    if x {
        id.to_tex();
    }
    //println!("{}",(c1.string()))
    
    
}
