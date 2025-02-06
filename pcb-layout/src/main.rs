use std::{collections::{btree_map, BTreeMap}, vec};
use num::{cast::AsPrimitive, integer::gcd, ToPrimitive};
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
        let start_x = self.x1/disc;
        let start_y = self.y1/disc;
        let end_x = self.x2/disc;
        let end_y = self.y2/disc;
        let mut cur_x = start_x;
        let mut cur_y = start_y;
        while cur_x <  end_x{
            while cur_y < end_y{
                let tmp_dict = (cur_x.try_into().unwrap(), cur_y.try_into().unwrap());
                ret_btree.insert(tmp_dict, value);
                cur_y += 1;
            }
            cur_x += 1;
            cur_y = start_y;

        }
        
        ret_btree
    }
}
struct Placement{
    components : Vec<Component>,
    placement_area: Bbox
    
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

}

fn swap(i:Individual) -> Individual{
    let a: usize = 1;
    let b: usize = 2; 

    i
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
    let placement_area = Bbox::new(0, 24, 0, 24);
    let mut boxx= Bbox::new(0,2,0,4);
    let mut c1 = Component{refdes: "C1".to_string(), bbox:boxx, rotation:0};
    let mut box2= Bbox::new(4,8,6,8);
    let mut c2 = Component{refdes: "C2".to_string(), bbox:box2, rotation:0};
    let mut box3= Bbox::new(4,12,0,6);
    let mut c3 = Component{refdes: "C3".to_string(), bbox:box3, rotation:0};
    //c1.move_comp( 10, 11);
    //c1.rotate_comp(90);
    let mut comps:Vec<Component> = vec![c1,c2,c3];
    let pl = Placement{components: comps, placement_area: placement_area };
    let mut id = Individual::new(pl);
    id.to_tex();
    //println!("{}",(c1.string()))
    
    
}
