use plotters::prelude::LogScalable;
use plotters::prelude::*;
use std::collections::BTreeMap;
use std::f64::consts::PI;

#[derive(Debug, Copy, Clone)]
pub struct Bbox {
    pub x1: f64,
    pub x2: f64,
    pub y1: f64,
    pub y2: f64,
    pub centerx: f64,
    pub centery: f64,
}

impl Bbox {
    pub fn new(x1: f64, x2: f64, y1: f64, y2: f64) -> Bbox {
        Bbox {
            x1,
            x2,
            y1,
            y2,
            centerx: x1 + ((x1 - x2).abs() / 2.0),
            centery: y1 + ((y1 - y2).abs() / 2.0),
        }
    }
    pub fn recenter(&mut self) {
        self.centerx = self.x1 + ((self.x1 - self.x2).abs() / 2.0);
        self.centery = self.y1 + ((self.y1 - self.y2).abs() / 2.0);
    }
    
    pub fn get_width(&self) -> f64 {
        (self.x1 - self.x2).abs()
    }
    pub fn get_height(&self) -> f64 {
        (self.y1 - self.y2).abs()
    }
    ///Helper function for plotting
    pub fn plot(&self, color: &RGBColor) -> Rectangle<(f64, f64)> {
        let ul: (f64, f64) = (self.x1, self.y2);
        let br: (f64, f64) = (self.x2, self.y1);
        Rectangle::new([ul, br], color.filled())
    }
    ///Checks to see if this is outside of another bbox
    pub fn is_out_of_bounds(&self, outer: &Bbox) -> bool {
        self.x1 < outer.x1 || self.x2 > outer.x2 || self.y1 < outer.y1 || self.y2 > outer.y2
    }
    ///Checks to see if this overlaps with another bbox
    pub fn does_overlap(&self, other: &Bbox) -> bool {
        let no_overlap =
            !(self.x2 < other.x1 || other.x2 < self.x1 || self.y2 < other.y1 || other.y2 < self.y1);
        no_overlap || (self.centerx == other.centerx && self.centery == other.centery)
    }

    pub fn get_center(&self) -> (f64, f64) {
        //self.bbox.recenter();
        (self.centerx, self.centery)
    }
    /// Rotates  ```angle degrees``` around the center
    ///
    pub fn rotate(&mut self, angle_degrees: f64) {
        self.recenter();
        let angle_radians = angle_degrees * PI / 180.0;

        let rotate_point = |x: f64, y: f64| -> (f64, f64) {
            let dx = x - self.centerx;
            let dy = y - self.centery;
            let sin = angle_radians.sin();
            let cos = angle_radians.cos();

            let new_x = dx * cos - dy * sin;
            let new_y = dx * sin + dy * cos;
            (self.centerx + new_x.round(), self.centery + new_y.round())
        };

        let ll = rotate_point(self.x1, self.y1);
        let ur = rotate_point(self.x2, self.y2);
        self.x1 = if ll.0 < ur.0 { ll.0 } else { ur.0 };
        self.y1 = if ll.1 < ur.1 { ll.1 } else { ur.1 };
        self.x2 = if ll.0 > ur.0 { ll.0 } else { ur.0 };
        self.y2 = if ll.1 > ur.1 { ll.1 } else { ur.1 };
        //self.recenter();
    }
    /// Rotates a bbox a ```angle degrees``` around a ```x, y```
    pub fn rotate_around_point(&mut self, angle_degrees: f64, centerx: f64, centery: f64) {
        self.recenter();
        let angle_radians = angle_degrees * PI / 180.0;

        let rotate_point = |x: f64, y: f64| -> (f64, f64) {
            let dx = x - centerx;
            let dy = y - centery;
            let sin = angle_radians.sin();
            let cos = angle_radians.cos();

            let new_x = dx * cos - dy * sin;
            let new_y = dx * sin + dy * cos;
            (centerx + new_x.round(), centery + new_y.round())
        };

        let ll = rotate_point(self.x1, self.y1);
        let ur = rotate_point(self.x2, self.y2);
        self.x1 = if ll.0 < ur.0 { ll.0 } else { ur.0 };
        self.y1 = if ll.1 < ur.1 { ll.1 } else { ur.1 };
        self.x2 = if ll.0 > ur.0 { ll.0 } else { ur.0 };
        self.y2 = if ll.1 > ur.1 { ll.1 } else { ur.1 };
        self.recenter();
    }
    /// Moves a bbox the x,y provided
    ///
    /// Also recenters
    pub fn move_bbx(&mut self, x: f64, y: f64) {
        self.x1 += x;
        self.y1 += y;
        self.x2 += x;
        self.y2 += y;
        self.recenter();
    }
}
#[derive(Debug, Clone)]
pub struct Placement {
    pub components: Vec<Component>,
    pub placement_area: Bbox,
    pub net_map: BTreeMap<i32, String>,
}
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Pin {
    pub refdes: String,
    pub net: i32,
    pub bbox: Bbox,
}
impl Pin {
    /// Moves a pin the delta and recenters it bbox
    pub fn move_pin(&mut self, x: f64, y: f64) {
        self.bbox.x1 += x;
        self.bbox.y1 += y;
        self.bbox.x2 += x;
        self.bbox.y2 += y;
        self.bbox.recenter();
    }
}
#[derive(Debug, Clone)]
pub struct Component {
    pub refdes: String,
    pub bbox: Bbox,
    pub rotation: i32,
    pub pins: Vec<Pin>,
}
impl Component {
    /// Moves a component and its respective pins (learning the problem with move being protected)
    /// This moves an amount - to move to a point use the ```move_to``` function
    /// Also recenters
    pub fn move_comp(&mut self, x: f64, y: f64) {
        self.bbox.x1 += x;
        self.bbox.y1 += y;
        self.bbox.x2 += x;
        self.bbox.y2 += y;
        for pin in &mut self.pins {
            pin.move_pin(x, y);
        }
        self.bbox.recenter();
    }
    /// Rotates a comp around its center and also handles rotating the pins
    /// Also recenters
    pub fn rotate_comp(&mut self, delta: i32) {
        self.bbox.recenter();
        self.rotation += delta;
        self.rotation %= 360;
        for pin in &mut self.pins {
            pin.bbox
                .rotate_around_point(delta.as_f64(), self.bbox.centerx, self.bbox.centery);
        }
        self.bbox.rotate(delta.as_f64());
        self.bbox.recenter();
    }
    /// Helper function for when you know what point and dont want to bother calculating delta
    pub fn move_to(&mut self, x: f64, y: f64) {
        self.bbox.recenter();
        let delta_x = x - self.bbox.x1;
        let delta_y = y - self.bbox.y1;
        self.move_comp(delta_x, delta_y);
    }
    /// Helper to set the refdes
    pub fn set_refdes(&mut self, new_ref: String) {
        self.refdes = new_ref.clone();
        for pin in &mut self.pins {
            pin.refdes = new_ref.clone();
        }
    }
}
///Calculates the net by net hpwl
/// Haven't looked at this since I wrote it. Definetely will need work
/// Currently GND is hardcoded (bad)
pub fn hpwl(comps: &Vec<Component>) -> f64 {
    let mut max_x = -1000000.0;
    let mut min_x = 100000.0;
    let mut max_y = -100000.0;
    let mut min_y = 100000.0;
    let mut pin_by_node: BTreeMap<i32, Vec<&Pin>> = BTreeMap::new();
    let mut total_wl = 0.0;
    let ignore_gnd = true;
    for i in comps {
        for pin in &i.pins {
            if ignore_gnd && pin.net != 11 {
                if let std::collections::btree_map::Entry::Vacant(e) = pin_by_node.entry(pin.net) {
                    e.insert(vec![pin]);
                } else {
                    let new_vec = pin_by_node.get_mut(&pin.net).unwrap();
                    new_vec.push(pin);
                    //pin_by_node.insert(pin.net, new_vec);
                }
            }
        }
    }
    for pins in pin_by_node.values() {
        for i in pins {
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
        total_wl += net_bbox.get_height() + net_bbox.get_width();
    }
    total_wl
}
/// This uses just max size (not chull which is more accurate)
pub fn placement_area(comps: &Vec<Component>) -> f64 {
    let mut max_x = 0.0;
    let mut min_x = 1000000.0;
    let mut max_y = 0.0;
    let mut min_y = 1000000.0;
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
    (max_x - min_x) * (max_y - min_y)
}
//Makes sure no components overlap
pub fn is_valid(comps: &Vec<Component>) -> f64 {
    let mut retur = 1.0;
    for comp_i in comps {
        for comp_j in comps {
            if comp_i.refdes != comp_j.refdes && comp_i.bbox.does_overlap(&comp_j.bbox) {
                retur = 10000.0;
            }
        }
    }
    retur
}

impl Placement {
    ///shifts a placement to an xy and returns the new xy
    pub fn shift_placement(&mut self, x: f64, y: f64) -> (f64, f64) {
        let delta_x = x - self.placement_area.x1;
        let delta_y = y - self.placement_area.y1;
        self.placement_area.move_bbx(delta_x, delta_y);
        for comp in &mut self.components {
            comp.move_comp(delta_x, delta_y);
        }

        (
            self.placement_area.x1 + delta_x,
            self.placement_area.y1 + delta_y,
        )
    }
}
