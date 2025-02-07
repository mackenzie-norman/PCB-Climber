use std::{collections::{btree_map, BTreeMap}, vec};

#[derive(Debug)]
pub struct Bbox{
    pub x1:i32,
    pub x2:i32,
    pub y1:i32,
    pub y2:i32,
    pub centerx: i32,
    pub centery: i32
}

impl Bbox{
    pub fn new(x1:i32 ,x2:i32 ,y1:i32 ,y2:i32 ) -> Bbox{
        Bbox { x1: x1, x2: x2, y1: y1, y2: y2, centerx: (x1-x2).abs() / 2, centery: (y1-y2).abs()/2 }
    }
    pub fn get_width(&self) -> usize{
        return (self.x1-self.x2).unsigned_abs().try_into().unwrap()

    }
    pub fn get_height(&self) -> usize{
        return (self.y1-self.y2).unsigned_abs().try_into().unwrap()
        
    }
    pub fn as_btree(&self, disc:i32, value:usize) -> BTreeMap<(usize, usize), usize> {
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
pub struct Placement{
    pub components : Vec<Component>,
    pub placement_area: Bbox
    
}
#[derive(Debug)]
pub struct Component{
    pub refdes : String,
    pub bbox: Bbox,
    pub rotation: i32,
}
impl Component{
    fn string(&self) -> String{
        return  self.refdes.clone() + " at (" + &self.bbox.centerx.to_string()  +"," + &self.bbox.centery.to_string() + ")";
    }
    pub fn move_comp(&mut self,x:i32 ,y:i32){
        self.bbox.x1 += x;
        self.bbox.y1 += y;
        self.bbox.x2 += x;
        self.bbox.y2 += y;
    }
    pub fn rotate_comp(& mut self, delta: i32){
        self.rotation += delta;
    }
    pub fn get_width(&self) -> usize{
        return self.bbox.get_width()
    }
    pub fn get_height(&self) -> usize{
        return self.bbox.get_height()
    }

}