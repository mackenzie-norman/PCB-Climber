use std::{collections::BTreeMap, vec};
use num::integer::lcm;
//CHAT GPT CODE JUST TO TEST
fn lcm_of_vector(nums: &[usize]) -> usize {

    let mut result = nums[0]; // Initialize with the first element

    for num in nums.iter().skip(1) {

        result = lcm(result, *num); // Calculate LCM for each pair

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
    discretization: f32,
}
impl Individual{
    fn new( pl: Placement) -> Self{
        //For now lets just say its a 6 x 6
        let mut sizes = Vec::new();
        for a in pl.components{
            sizes.push(a.get_height());
            sizes.push(a.get_width());
        }
        //println!("{:?}", sizes);
        let disc = lcm_of_vector(&sizes);

        let mut a:BTreeMap<(usize, usize), usize > = BTreeMap::new();
        for c in pl.components{
            let st = c.bbox.x1;
            let end = c.bbox.x2;
        }
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
