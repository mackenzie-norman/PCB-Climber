use std::{collections::BTreeMap, vec};
mod plcmnt;
use num::{cast::AsPrimitive, ToPrimitive};
use plcmnt::{gcd_of_vector, hpwl, Bbox, Component, Placement};
use rand::prelude::*;
use colored::Colorize;


fn random_rotation() -> i32 {
    // Get an RNG:
    let mut rng = rand::rng();
    let opts = [90, 180, 270];
    let choice = opts.choose(&mut rng).unwrap();
    *choice
}
struct Individual {
    chromosone: Vec<usize>,
    comp_list: Vec<Component>,
    discretization: usize,
    x_sz: usize,
    y_sz: usize,
}

impl Individual {
    fn new(pl: Placement) -> Self {
        let mut sizes = Vec::new();
        for a in &pl.components {
            sizes.push(a.get_height());
            sizes.push(a.get_width());
        }
        //println!("{:?}", sizes);
        let disc = gcd_of_vector(&sizes);

        let mut a: BTreeMap<(usize, usize), usize> = BTreeMap::new();
        let mut count: usize = 1;
        for c in &pl.components {
            let mut c_space = c.bbox.as_btree(disc.try_into().unwrap(), count);
            a.append(&mut c_space);
            count += 1usize;
        }

        let mut y_end: usize = (pl.placement_area.y2 / disc.to_i32().unwrap())
            .try_into()
            .unwrap();
        let mut x_end: usize = (pl.placement_area.x2 / disc.to_i32().unwrap())
            .try_into()
            .unwrap();
        x_end += 10;
        y_end += 10;

        let mut arr: Vec<usize> = vec![0; x_end * y_end];
        let tmp_i = Individual {
            chromosone: Vec::new(),
            comp_list: Vec::new(),
            discretization: disc,
            x_sz: x_end,
            y_sz: y_end,
        };
        for k in a.iter() {
            let index: usize = tmp_i.tuple_to_index(*k.0);
            arr[index] = *k.1;
        }

        Individual {
            chromosone: arr,
            comp_list: pl.components,
            discretization: disc,
            x_sz: x_end,
            y_sz: y_end,
        }
    }
    fn tuple_to_index(&self, tple: (usize, usize)) -> usize {
        (self.x_sz - tple.0) + (tple.1 * self.x_sz)
    }
    fn pretty_print(&self) {
        let mut c = 0;
        for i in (&self.chromosone).into_iter().rev() {
            if *i != 0 {
                print!("{}", i.to_string().blue());
            } else {
                print!("{}", "\u{25A0}".green());
            }
            c += 1;
            if c >= self.x_sz {
                c = 0;
                println!("");
            }
        }
    }
    /*
    fn is_valid(&self) -> bool {
        let mut valid: bool = true;
        let mut a: BTreeMap<(usize, usize), usize> = BTreeMap::new();
        let mut count: usize = 1;

        for c in &self.comp_list {
            let mut c_space = (*c)
                .bbox
                .as_btree(self.discretization.try_into().unwrap(), count);
            count += 1usize;

            for k in c_space.iter() {
                let x = k.0 .0;
                let y = k.0 .1;
                let val = *k.1;
                let try_val = self.chromosone[y][x];
                if val != try_val {
                    return false;
                }
            }
        }
        valid
    }
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
    fn swap(&mut self, a: usize, b: usize) {
        let mut old_coords: BTreeMap<(usize, usize), usize> = BTreeMap::new();
        let mut new_coords: BTreeMap<(usize, usize), usize> = BTreeMap::new();

        //We need to zero, so lets grab the coords and also hold on to them
        let a_comp = &(self.comp_list[a - 1]);
        let mut c_space = (a_comp)
            .bbox
            .as_btree(self.discretization.try_into().unwrap(), 0);
        let old_a_loc = (a_comp.bbox.x1, a_comp.bbox.y1);
        old_coords.append(&mut c_space);

        let b_comp = &(self.comp_list[b - 1]);
        let mut c_space = (b_comp)
            .bbox
            .as_btree(self.discretization.try_into().unwrap(), 0);
        let old_b_loc = (b_comp.bbox.x1, b_comp.bbox.y1);
        old_coords.append(&mut c_space);

        //Now we want to swap locations
        let mut a_comp = &mut (self.comp_list[a - 1]);
        a_comp.move_to(old_b_loc.0, old_b_loc.1);
        let mut c_space = (a_comp)
            .bbox
            .as_btree(self.discretization.try_into().unwrap(), a);
        new_coords.append(&mut c_space);

        let mut b_comp = &mut (self.comp_list[b - 1]);
        b_comp.move_to(old_a_loc.0, old_a_loc.1);
        let mut c_space = (b_comp)
            .bbox
            .as_btree(self.discretization.try_into().unwrap(), b);
        new_coords.append(&mut c_space);
        for k in old_coords.iter() {
            let val = k.1;
            let idx = self.tuple_to_index(*k.0);
            self.chromosone[idx] = *val;
        }
        for k in new_coords.iter() {
            let val = k.1;
            let idx = self.tuple_to_index(*k.0);
            self.chromosone[idx] = *val;
        }
    }

    fn move_to_new(&mut self, a: usize) {
        let mut old_coords: BTreeMap<(usize, usize), usize> = BTreeMap::new();
        let mut new_coords: BTreeMap<(usize, usize), usize> = BTreeMap::new();
        let mut bad_coords: BTreeMap<usize, usize> = BTreeMap::new();
        let mut rng = rand::rng();
        let x = rng.random_range(0..((self.x_sz - 4)* self.discretization));
        let y = rng.random_range(0..((self.y_sz -4) * self.discretization));
        //We need to zero, so lets grab the coords and also hold on to them
        //let a: usize = 2;

        let mut a_comp = &mut (self.comp_list[a - 1]);
        let mut c_space = (a_comp)
            .bbox
            .as_btree(self.discretization.try_into().unwrap(), 0);
        old_coords.append(&mut c_space);
        a_comp.move_to(x.to_i32().unwrap(), y.to_i32().unwrap());
        let mut c_space = (a_comp)
            .bbox
            .as_btree(self.discretization.try_into().unwrap(), a);
        new_coords.append(&mut c_space);
        let mut worked = true;
        for k in new_coords.iter() {
            let idx = self.tuple_to_index(*k.0);
            if self.chromosone[idx] != 0 && self.chromosone[idx] != a {
                worked = false;
            }
            bad_coords.insert(idx, self.chromosone[idx] );
            //self.chromosone[idx] = *val; 
        }
        if worked {
            //println!("it wokred");
            for k in old_coords.iter() {
                let val = k.1;
                let idx = self.tuple_to_index(*k.0);
                self.chromosone[idx] = *val;
            }
            for k in bad_coords.iter() {
                let idx = *k.0;
                
                self.chromosone[idx] = a;
                
            }

        } else {
            /* 
            for k in bad_coords.iter() {
                let idx = *k.0;
                
                self.chromosone[idx] = *k.1;
                
            }
            */
        }
    }

    fn score(&mut self) -> usize {
        hpwl(&mut self.comp_list)
    }

    fn rotate(&mut self, a: usize, rotation: i32) {
        let mut old_coords: BTreeMap<(usize, usize), usize> = BTreeMap::new();
        let mut new_coords: BTreeMap<(usize, usize), usize> = BTreeMap::new();

        //We need to zero, so lets grab the coords and also hold on to them
        //let a: usize = 2;

        let mut a_comp = &mut (self.comp_list[a - 1]);
        let mut c_space = (a_comp)
            .bbox
            .as_btree(self.discretization.try_into().unwrap(), 0);
        old_coords.append(&mut c_space);
        a_comp.rotate_comp(rotation);

        let mut c_space = (a_comp)
            .bbox
            .as_btree(self.discretization.try_into().unwrap(), a);
        new_coords.append(&mut c_space);
        let mut worked = true;
        for k in old_coords.iter() {
            let val = k.1;
            let idx = self.tuple_to_index(*k.0);
            if self.chromosone[idx] != 0 && self.chromosone[idx] != a {
                worked = false;
            }
            self.chromosone[idx] = *val;
        }
        if worked {
            for k in new_coords.iter() {
                let val = k.1;
                let idx = self.tuple_to_index(*k.0);
                self.chromosone[idx] = *val;
            }
        } else {
            for k in old_coords.iter() {
                let idx = self.tuple_to_index(*k.0);
                self.chromosone[idx] = a;
            }
        }
    }
}

fn main() {
    let placement_area = Bbox::new(0, 24, 0, 24);
    let boxx = Bbox::new(0, 2, 0, 4);
    let mut c1 = Component {
        refdes: "C1".to_string(),
        bbox: boxx,
        rotation: 0,
    };
    let box2 = Bbox::new(4, 8, 6, 8);
    let mut c2 = Component {
        refdes: "C2".to_string(),
        bbox: box2,
        rotation: 0,
    };
    let box3 = Bbox::new(4, 13, 0, 6);
    let mut c3 = Component {
        refdes: "C3".to_string(),
        bbox: box3,
        rotation: 0,
    };
    c1.move_comp(6, 6);
    c2.move_comp(6, 6);
    c3.move_comp(6, 6);
    //c1.move_comp( 10, 11);
    //c1.rotate_comp(90);
    let comps: Vec<Component> = vec![c1, c2, c3];
    let mut pl = Placement {
        components: comps,
        placement_area: placement_area,
    };

    let mut id = Individual::new(pl);
    //id.rotate(2, 90);
    //id.rotate(2, 90);
    //id.rotate(2, 270);
    //id.rotate(3, 0);
    //id.rotate(2, 180);
    id.pretty_print();
    println!("{}", id.score());
    //id.swap(1, 3);
    for _ in 0..100 {
        //id.swap(1, 2);
        id.move_to_new(1);
        //id.swap(1, 2);
        id.move_to_new(2);
        //id.move_to_new(1);
    }
    println!("{}", id.score());
    id.pretty_print();

    /*
    //id.swap();

    println!("{}", id.score());

    let x = id.is_valid();
    if x {
        id.to_tex();
    }
    */
    //println!("{}",(c1.string()))
}
