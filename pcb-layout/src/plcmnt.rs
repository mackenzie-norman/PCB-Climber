#![allow(dead_code)]
use num::{integer::gcd, ToPrimitive};
use plotters::prelude::LogScalable;
use std::collections::BTreeMap;
use std::f64::consts::PI;
pub fn gcd_of_vector(nums: &[usize]) -> usize {
    let mut result = nums[0]; // Initialize with the first element

    for num in nums.iter().skip(1) {
        result = gcd(result, *num); // Calculate LCM for each pair
    }

    result
}
#[derive(Debug , Copy, Clone)]
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
            x1,
            x2,
            y1,
            y2,
            centerx: x1 + ((x1 - x2).abs() / 2),
            centery: y1 + ((y1 - y2).abs() / 2),
        }
    }
    pub fn recenter(&mut self) {
        self.centerx = self.x1 + ((self.x1 - self.x2).abs() / 2);
        self.centery = self.y1 + ((self.y1 - self.y2).abs() / 2);
    }
    
    pub fn get_width(&self) -> usize {
        (self.x1 - self.x2).unsigned_abs().try_into().unwrap()
    }
    pub fn get_height(&self) -> usize {
        (self.y1 - self.y2).unsigned_abs().try_into().unwrap()
    }
    pub fn get_width_fl(&self) -> u32 {
        (self.x1 - self.x2).unsigned_abs()
    }
    pub fn get_height_fl(&self) -> u32 {
        (self.y1 - self.y2).unsigned_abs()
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
    pub fn is_out_of_bounds(&self, outer: &Bbox) -> bool {
        self.x1 < outer.x1 || self.x2 > outer.x2 || self.y1 < outer.y1 || self.y2 > outer.y2
    }
    pub fn does_overlap(&self, other: &Bbox) -> bool{
        let no_overlap = !(self.x2 < other.x1 || other.x2 < self.x1 || self.y2 < other.y1 || other.y2 < self.y1) ;
        no_overlap || (self.centerx == other.centerx && self.centery == other.centery)

    }
    pub fn get_center(& self) -> (i32, i32) {
        //self.bbox.recenter();
        (self.centerx, self.centery)
    }
    /// Rotates around the x1,y1 to avoid nasty discretization issues.
    /// 
    pub fn rotate(&mut self, angle_degrees: f64)  {
        self.recenter();
        let angle_radians = angle_degrees * PI / 180.0;

        let rotate_point = |x: i32, y: i32| -> (i32, i32) {
            let dx = ( x - self.centerx  ) as f64;
            let dy = ( y - self.centery  ) as f64;
            let sin = angle_radians.sin();
            let cos = angle_radians.cos();

            let new_x = dx *cos  - dy * sin;
            let new_y = dx * sin  + dy * cos;
            (self.centerx + new_x.round() as i32, self.centery + new_y.round() as i32)
        };

        let ll = rotate_point(self.x1, self.y1);
        let ur = rotate_point(self.x2, self.y2);
        self.x1 =  if ll.0 < ur.0{ ll.0} else { ur.0};
        self.y1 = if ll.1 < ur.1{ ll.1} else { ur.1};
        self.x2 =  if ll.0 > ur.0{ ll.0} else { ur.0};
        self.y2 = if ll.1 > ur.1{ ll.1} else { ur.1};
        //self.recenter();
    }
    pub fn rotate_around_point(&mut self, angle_degrees: f64, centerx:i32, centery:i32)  {
        self.recenter();
        let angle_radians = angle_degrees * PI / 180.0;

        let rotate_point = |x: i32, y: i32| -> (i32, i32) {
            let dx = ( x - centerx  ) as f64;
            let dy = ( y - centery  ) as f64;
            let sin = angle_radians.sin();
            let cos = angle_radians.cos();

            let new_x = dx *cos  - dy * sin;
            let new_y = dx * sin  + dy * cos;
            (centerx + new_x.round() as i32, centery + new_y.round() as i32)
        };

        let ll = rotate_point(self.x1, self.y1);
        let ur = rotate_point(self.x2, self.y2);
        self.x1 =  if ll.0 < ur.0{ ll.0} else { ur.0};
        self.y1 = if ll.1 < ur.1{ ll.1} else { ur.1};
        self.x2 =  if ll.0 > ur.0{ ll.0} else { ur.0};
        self.y2 = if ll.1 > ur.1{ ll.1} else { ur.1};
        self.recenter();
    }
    /* 
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
    */
}
#[derive(Debug ,  Clone)]
pub struct Placement {
    pub components: Vec<Component>,
    pub placement_area: Bbox,
}
#[derive(Debug ,  Clone)]
pub struct Pin{
    pub refdes: String,
    pub net: i32,
    pub bbox: Bbox,

}
impl Pin{
    pub fn move_pin(&mut self, x: i32, y: i32) {
        self.bbox.x1 += x;
        self.bbox.y1 += y;
        self.bbox.x2 += x;
        self.bbox.y2 += y;
        self.bbox.recenter();
    }

}
#[derive(Debug ,  Clone)]
pub struct Component {
    pub refdes: String,
    pub bbox: Bbox,
    pub rotation: i32,
    pub pins: Vec<Pin>
}
impl Component {
    fn string(&self) -> String {
        self.refdes.clone()
            + " at ("
            + &self.bbox.centerx.to_string()
            + ","
            + &self.bbox.centery.to_string()
            + ")"
    }
    pub fn move_comp(&mut self, x: i32, y: i32) {
        self.bbox.x1 += x;
        self.bbox.y1 += y;
        self.bbox.x2 += x;
        self.bbox.y2 += y;
        for pin in &mut self.pins{
            pin.move_pin(x, y);
        }
        self.bbox.recenter();
    }
    pub fn rotate_comp(&mut self, delta: i32) {
        self.bbox.recenter();
        self.rotation += delta ;
        self.rotation %= 360;
        self.bbox.rotate(delta.as_f64());
        for pin in &mut self.pins{
            pin.bbox.rotate_around_point(delta.as_f64(), self.bbox.centerx, self.bbox.centery);
        }
        self.bbox.recenter();
    }
    pub fn get_width(&self) -> usize {
        self.bbox.get_width()
    }
    pub fn get_height(&self) -> usize {
        self.bbox.get_height()
    }
    pub fn move_to(&mut self, x: i32, y: i32) {
        self.bbox.recenter();
        let delta_x = x - self.bbox.x1;
        let delta_y = y - self.bbox.y1;
        self.move_comp(delta_x, delta_y);
        
    }
    pub fn get_center(& self) -> (i32, i32) {
        //self.bbox.recenter();
        (self.bbox.centerx, self.bbox.centery)
    }
    pub fn try_move_to(& self, x: i32, y: i32 ) -> Bbox {
        let delta_x = x - self.bbox.x1;
        let delta_y = y - self.bbox.y1;
        Bbox::new(
        self.bbox.x1 + delta_x,
        self.bbox.y1 + delta_y,
        self.bbox.x2 + delta_x,
        self.bbox.y2 + delta_y)


    }
    pub fn set_refdes(& mut self, new_ref: String){
        self.refdes = new_ref.clone();
        for pin in &mut self.pins{
            pin.refdes = new_ref.clone();
        }

    }
    pub fn is_negative( &self) -> bool{
        self.bbox.x1 < 0 || self.bbox.y1 < 0
    }
}
///This assumes all comps are on the same net lol
pub fn hpwl(comps: & Vec<Component>) -> u32 {
    let mut max_x = -1000000000;
    let mut min_x = 100000;
    let mut max_y = -100000000;
    let mut min_y = 100000;
    let mut pin_by_node: BTreeMap<i32, Vec<&Pin>> = BTreeMap::new();
    let mut total_wl = 0;
    let ignore_gnd = true;
    for i in comps {
        for pin in &i.pins{
            if ignore_gnd && pin.net != 0{

            if let std::collections::btree_map::Entry::Vacant(e) = pin_by_node.entry(pin.net) {
                e.insert(vec![pin]);
            } else {
                let new_vec =pin_by_node.get_mut(&pin.net).unwrap();
                new_vec.push(pin);
                //pin_by_node.insert(pin.net, new_vec);
            }
            }
        }
    }
    for pins in pin_by_node.values(){
        for i in pins{
        let (x, y) = i.bbox.get_center();
        if x > max_x {
            max_x = x
        };
        if y > max_y {
            max_y = y
        };
        if x < min_x {
            min_x = x
        };
        if y < min_y {
            min_y = y
        };
    }
    let net_bbox = Bbox::new(min_x, max_x, min_y, max_y);
    total_wl += net_bbox.get_height_fl() + net_bbox.get_width_fl();
    }
    total_wl
}
/// This uses just max size (not chull which is more accurate)
pub fn placement_area(comps: & Vec<Component>) -> u32 {
    let mut max_x = 0;
    let mut min_x = 1000000;
    let mut max_y = 0;
    let mut min_y = 1000000;
    for i in comps {
        //TODO         
        let (x, y) = (i.bbox.x1, i.bbox.y1);
        if x > max_x {
            max_x = x
        };
        if y > max_y {
            max_y = y
        };
        if x < min_x {
            min_x = x
        };
        if y < min_y {
            min_y = y
        };

        let (x, y) = (i.bbox.x2, i.bbox.y2);
        if x > max_x {
            max_x = x
        };
        if y > max_y {
            max_y = y
        };
        if x < min_x {
            min_x = x
        };
        if y < min_y {
            min_y = y
        };
    }
    
    //println!("{:?}", net_bbox);
    ((max_x - min_x) * (max_y -min_y)).try_into().unwrap()
}
pub fn is_valid (comps: & Vec<Component>) -> u32 {
    let mut retur = 1;
    for comp_i in comps{
        for comp_j in comps{
            if comp_i.refdes != comp_j.refdes && comp_i.bbox.does_overlap(&comp_j.bbox){
                retur = 10000;
            }
        }
    }
    retur
}

impl Placement {
    pub fn array_size(&mut self) -> (usize, usize) {
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

        let mut y_end: usize = (self.placement_area.y2 / disc.to_i32().unwrap())
            .try_into()
            .unwrap();
        let mut x_end: usize = (self.placement_area.x2 / disc.to_i32().unwrap())
            .try_into()
            .unwrap();
        x_end += 10;
        y_end += 10;
        (x_end * y_end, disc)
    }
}
