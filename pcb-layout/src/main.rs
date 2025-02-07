use std::{collections::{btree_map, BTreeMap}, vec};
use num::{cast::AsPrimitive, integer::gcd, ToPrimitive};
mod plcmnt;
use plcmnt::{Component, Bbox, Placement};
//CHAT GPT CODE JUST TO TEST
fn gcd_of_vector(nums: &[usize]) -> usize {

    let mut result = nums[0]; // Initialize with the first element

    for num in nums.iter().skip(1) {

        result = gcd(result, *num); // Calculate LCM for each pair

    }

    result

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
        for c in pl.components{
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
            csone_vec.insert(0,t_v)
        }
        Individual{chromosone: csone_vec, comp_list: Vec::new(), discretization: disc}
    }
    fn to_tex(&self) {
       for i in &self.chromosone{
        println!("{:?}", i);
       } 
    }
    fn is_valid(&self) -> bool{
        let mut valid: bool = true;
        let mut a:BTreeMap<(usize, usize), usize > = BTreeMap::new();
        let mut count: usize = 1;
        println!("{:?}", self.comp_list);
        for c in &self.comp_list{
            let mut c_space = (*c).bbox.as_btree(self.discretization.try_into().unwrap(), count);
            a.append(&mut c_space);
            count += 1usize;
        }
        println!("{:?}", a);
        for k in a.iter(){
            let x = k.0.0 ;
            println!("{}",x);
        }
        valid 
    }
}

fn swap(i:Individual) -> Individual{
    let a: usize = 1;
    let b: usize = 2; 

    i
}

fn main() {
    let placement_area = Bbox::new(0, 24, 0, 24);
    let  boxx= Bbox::new(0,2,0,4);
    let  c1 = Component{refdes: "C1".to_string(), bbox:boxx, rotation:0};
    let  box2= Bbox::new(4,8,6,8);
    let  c2 = Component{refdes: "C2".to_string(), bbox:box2, rotation:0};
    let  box3= Bbox::new(4,12,0,6);
    let  c3 = Component{refdes: "C3".to_string(), bbox:box3, rotation:0};
    //c1.move_comp( 10, 11);
    //c1.rotate_comp(90);
    let  comps:Vec<Component> = vec![c1,c2,c3];
    let pl = Placement{components: comps, placement_area: placement_area };
    let mut id = Individual::new(pl);
    id.to_tex();
    let x = id.is_valid();
    if x {
        id.to_tex();
    }
    //println!("{}",(c1.string()))
    
    
}
