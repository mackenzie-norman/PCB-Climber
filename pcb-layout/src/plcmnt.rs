use std::{collections::BTreeMap, vec};
use std::f64::consts::PI;
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
    pub fn rotate(&mut self, angle_degrees: f64)  {
        self.recenter();
        let angle_radians = angle_degrees * PI / 180.0;

        let rotate_point = |x: i32, y: i32| -> (i32, i32) {
            let dx = ( self.centerx - x ) as f64;
            let dy = ( self.centery -y ) as f64;
            let new_x = self.centerx as f64 + dx * angle_radians.cos() - dy * angle_radians.sin();
            let new_y = self.centery as f64 + dx * angle_radians.sin() + dy * angle_radians.cos();
            (new_x.round() as i32, new_y.round() as i32)
        };

        let ll = rotate_point(self.x1, self.y1);
        let ur = rotate_point(self.x2, self.y2);
        println!("{},{},{},{}" , ll.0, ll.1,ur.0,ur.1);
        self.x1 = ll.0;
        self.y1 = ll.1;
        self.x2 = ur.0;
        self.y2 = ur.1;
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
        self.bbox.recenter();
    }
    pub fn rotate_comp(&mut self, delta: i32) {
        self.rotation += delta;
        self.bbox.rotate(delta as f64);
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
}
