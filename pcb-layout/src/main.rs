use std::{collections::{btree_map, BTreeMap}, vec};
use num::integer::gcd;
//CHAT GPT CODE JUST TO TEST
fn gcd_of_vector(nums: &[usize]) -> usize {

    let mut result = nums[0]; // Initialize with the first element

    for num in nums.iter().skip(1) {

        result = gcd(result, *num); // Calculate LCM for each pair

    }

    result

}
struct Bbox{
    x1:i32,
    x2:i32,
    y1:i32,
    y2:i32,
    centerx: i32,
    centery: i32
}

impl Bbox{
    fn new(x1:i32 ,x2:i32 ,y1:i32 ,y2:i32 ) -> Bbox{
        Bbox { x1: x1, x2: x2, y1: y1, y2: y2, centerx: (x1-x2).abs() / 2, centery: (y1-y2).abs()/2 }
    }
    fn get_width(&self) -> usize{
        return (self.x1-self.x2).unsigned_abs().try_into().unwrap()

    }
    fn get_height(&self) -> usize{
        return (self.y1-self.y2).unsigned_abs().try_into().unwrap()
        
    }
    fn as_btree(&self, disc:i32, value:usize) -> BTreeMap<(usize, usize), usize> {
        let mut ret_btree: BTreeMap <(usize,usize), usize> = BTreeMap::new();
        let start_x = self.x1;
        let start_y = self.y1;
        let end_x = self.x2;
        let end_y = self.y2;
        let mut cur_x = start_x;
        let mut cur_y = start_y;
        while cur_x <=  end_x{
            while cur_y <= end_y{
                let tmp_dict = (cur_x.try_into().unwrap(), cur_y.try_into().unwrap());
                ret_btree.insert(tmp_dict, value);
                cur_y += disc;
            }
            cur_x += disc;
            cur_y = start_y;

        }
        
        println!("{:?}", ret_btree);
        ret_btree
    }
}
struct Placement{
    components : Vec<Component>,

    
}
struct Component{
    refdes : String,
    bbox: Bbox,
    rotation: i32,
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
        for c in pl.components{
            let mut c_space = c.bbox.as_btree(disc.try_into().unwrap(), 1);
            println!("{:?}", c_space);
            a.append(&mut c_space);
            let st = c.bbox.x1;
            let end = c.bbox.x2;
        }
        println!("{:?}", a);
        
        let mut csone_vec = Vec::new();

        for y in 0..6{
            let mut t_v = Vec::new();
            for x in 0..6{
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
}
impl Component{
    fn string(&self) -> String{
        return  self.refdes.clone() + " at (" + &self.bbox.centerx.to_string()  +"," + &self.bbox.centery.to_string() + ")";
    }
    fn move_comp(&mut self,x:i32 ,y:i32){
        self.bbox.x1 += x;
        self.bbox.y1 += y;
        self.bbox.x2 += x;
        self.bbox.y2 += y;
    }
    fn rotate_comp(& mut self, delta: i32){
        self.rotation += delta;
    }
    fn get_width(&self) -> usize{
        return self.bbox.get_width()
    }
    fn get_height(&self) -> usize{
        return self.bbox.get_height()
    }

}
fn main() {
    println!("Hello, world!");
    let mut boxx= Bbox::new(0,2,0,4);
    let mut c1 = Component{refdes: "C1".to_string(), bbox:boxx, rotation:0};
    println!("{}",(c1.string()));
    //c1.move_comp( 10, 11);
    //c1.rotate_comp(90);
    let mut comps:Vec<Component> = Vec::new();
    comps.push(c1);

    let pl = Placement{components: comps };
    let mut id = Individual::new(pl);
    id.to_tex();
    //println!("{}",(c1.string()))
    
    
}
