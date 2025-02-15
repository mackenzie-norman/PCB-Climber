use std::{collections::BTreeMap};
use num::{cast::AsPrimitive, integer::gcd, ToPrimitive};


fn gcd_of_vector(nums: &[usize]) -> usize {
    let mut result = nums[0]; // Initialize with the first element

    for num in nums.iter().skip(1) {
        result = gcd(result, *num); // Calculate LCM for each pair
    }

    result
}
#[derive(Debug)]
pub struct Bbox {
    pub x1: i32,
    pub x2: i32,
    pub y1: i32,
    pub y2: i32,
    pub centerx: i32,
    pub centery: i32,
}

impl Bbox {
    pub fn new(x1: i32, x2: i32, y1: i32, y2: i32) -> Bbox {
        Bbox {
            x1: x1,
            x2: x2,
            y1: y1,
            y2: y2,
            centerx: (x1 - x2).abs() / 2,
            centery: (y1 - y2).abs() / 2,
        }
    }
    pub fn recenter(&mut self){
            self.centerx = (self.x1 - self.x2).abs() / 2;
            self.centery = (self.y1 - self.y2).abs() / 2;
    }
    pub fn get_width(&self) -> usize {
        return (self.x1 - self.x2).unsigned_abs().try_into().unwrap();
    }
    pub fn get_height(&self) -> usize {
        return (self.y1 - self.y2).unsigned_abs().try_into().unwrap();
    }
    pub fn as_btree(&self, disc: i32, value: usize) -> BTreeMap<(usize, usize), usize> {
        let mut ret_btree: BTreeMap<(usize, usize), usize> = BTreeMap::new();
        let start_x = self.x1 / disc;
        let start_y = self.y1 / disc;
        let end_x = self.x2 / disc;
        let end_y = self.y2 / disc;
        let mut cur_x = start_x;
        let mut cur_y = start_y;
        while cur_x < end_x {
            while cur_y < end_y {
                let tmp_dict = (cur_x.try_into().unwrap(), cur_y.try_into().unwrap());
                ret_btree.insert(tmp_dict, value);
                cur_y += 1;
            }
            cur_x += 1;
            cur_y = start_y;
        }
        ret_btree
    }
    /// Rotates around the x1,y1 to avoid nasty discretization issues.
    pub fn rotate(&mut self, angle: i32) {
        match angle {
            90 => {
                let height = self.get_height();
                let width = self.get_width();
                self.x2 = self.x1 + height as i32;
                self.y2 = self.y1 + width as i32;
            }
            180 => {
                let height = self.get_height();
                let width = self.get_width();
                //self.x1 = self.x2

                self.y2 = self.y1;
                self.y1 = self.y1 - height as i32;
                self.x2 = self.x1 + width as i32;
            }

            270 => {
                let height = self.get_height();
                let width = self.get_width();
                self.x2 = self.x1;
                self.y2 = self.y1;
                self.x1 = self.x2 - height as i32;
                self.y1 = self.y2 - width as i32;
            }
            _ => (),
        }
    }
}
pub struct Placement {
    pub components: Vec<Component>,
    pub placement_area: Bbox,
}
#[derive(Debug)]
pub struct Component {
    pub refdes: String,
    pub bbox: Bbox,
    pub rotation: i32,
}
impl Component {
    fn string(&self) -> String {
        return self.refdes.clone()
            + " at ("
            + &self.bbox.centerx.to_string()
            + ","
            + &self.bbox.centery.to_string()
            + ")";
    }
    pub fn move_comp(&mut self, x: i32, y: i32) {
        self.bbox.x1 += x;
        self.bbox.y1 += y;
        self.bbox.x2 += x;
        self.bbox.y2 += y;
    }
    pub fn rotate_comp(&mut self, delta: i32) {
        self.rotation += delta;
        self.bbox.rotate(delta);
    }
    pub fn get_width(&self) -> usize {
        return self.bbox.get_width();
    }
    pub fn get_height(&self) -> usize {
        return self.bbox.get_height();
    }
    pub fn move_to(&mut self, x: i32, y: i32) {
        let delta_x = x - self.bbox.x1;
        let delta_y = y - self.bbox.y1;
        self.move_comp(delta_x, delta_y);
    }
    pub fn get_center(&mut self) -> (i32,i32){
        self.bbox.recenter();
        (self.bbox.centerx, self.bbox.centery)
    }
}
///This assumes all comps are on the same net lol 
pub fn hpwl(comps: &mut Vec<Component>) -> usize{
    let mut max_x = 0;
    let mut min_x = 100000;
    let mut max_y = 0;
    let mut min_y = 100000;
    for i in comps{
        let (x,y) = i.get_center();
        if x > max_x{ max_x = x};
        if y > max_y{ max_y = y};
        if x < min_x{ min_x = x};
        if y < min_y{ min_y = y};
    }
    let net_bbox = Bbox::new(min_x, max_x, min_y, max_y);
    return net_bbox.get_height() + net_bbox.get_width();
        
}

impl Placement{
    pub fn array_size(&mut self) -> usize{
        let mut sizes = Vec::new();
        for a in &self.components {
            sizes.push(a.get_height());
            sizes.push(a.get_width());
        }
        //println!("{:?}", sizes);
        let disc = gcd_of_vector(&sizes);

        let mut a: BTreeMap<(usize, usize), usize> = BTreeMap::new();
        let mut count: usize = 1;
        for c in &self.components {
            let mut c_space = c.bbox.as_btree(disc.try_into().unwrap(), count);
            a.append(&mut c_space);
            count += 1usize;
        }

        let mut  y_end: usize = (self.placement_area.y2 / disc.to_i32().unwrap())
            .try_into()
            .unwrap();
        let mut x_end: usize = (self.placement_area.x2 / disc.to_i32().unwrap())
            .try_into()
            .unwrap();
        x_end +=  10;
        y_end += 10;
        return x_end * y_end;
    }

}