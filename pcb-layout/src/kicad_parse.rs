use crate::plcmnt;
use colored::Colorize;
use plcmnt::{Bbox, Component, Pin, Placement};
use plotters::prelude::*;
use rand::prelude::*;
use std::collections::BTreeMap;

use std::fs;

fn parse_kicad_line_to_floats(passed_str: &str) -> (f64, f64) {
    let x_y_str = passed_str.trim().replace(")", "");
    let x_y_vec: Vec<&str> = x_y_str.split(" ").collect();
    if x_y_vec.len() < 3 {
        println!("{:?}", x_y_vec);
    }

    (
        x_y_vec[1].parse::<f64>().unwrap(),
        x_y_vec[2].parse::<f64>().unwrap(),
    )
}
pub fn parse_file() -> Placement {
    // --snip--
    let file_path = "..\\arduino_kicad\\Arduino UNO.kicad_pcb";
    //let file_path = "..\\demo\\demo.kicad_pcb";
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut net_map: BTreeMap<i32, String> = BTreeMap::new();
    let mut comp_vec: Vec<Component> = Vec::new();
    //The layer used to calc the area of the foot print
    let footprint_layer = "CrtYd";
    let mut refdes = "some";
    let mut x1: f64 = 0.0;
    let mut y1: f64 = 0.0;
    let mut rotation: i32 = 0;
    let mut content_iter = contents.split("\n");
    let mut in_doc = true;
    let mut xs: Vec<f64> = Vec::new();
    let mut ys: Vec<f64> = Vec::new();
    while in_doc {
        let mut line = content_iter.next().unwrap_or_else(|| {
            in_doc = false;
            "bad"
        });
        if !in_doc {
            break;
        };
        //for line in contents.split("\n"){

        //print!("{}",line);
        //println!("{}",line);
        if line.starts_with("\t(net ") && line.trim_end().ends_with("\")") {
            //println!("{}",line);
            let words: Vec<&str> = line.split(" ").collect();
            let net_idx = words[1].parse::<i32>().unwrap();
            let net_name = words[2]
                .trim_end()
                .strip_suffix(|_: char| true)
                .unwrap()
                .replace("\"", "");
            //println!("{} : {}", net_idx, net_name);
            net_map.insert(net_idx, net_name);
        }
        if line.starts_with("\t(footprint ") {
            //println!("{}","new Device");
            line = content_iter.next().unwrap();
            line = content_iter.next().unwrap();
            let x_y_str = content_iter.next().unwrap().trim_end().replace(")", "");
            let x_y_vec: Vec<&str> = x_y_str.split(" ").collect();
            //println!("{:?}", x_y_vec);
            x1 = x_y_vec[1].parse::<f64>().unwrap();
            y1 = x_y_vec[2].parse::<f64>().unwrap();
            if x_y_vec.len() > 3 {
                rotation = x_y_vec[3].parse::<i32>().unwrap_or(0)
            } else {
                rotation = 0
            }
            //println!("{}", x_y_str);
            while !line.contains("Reference") {
                line = content_iter.next().unwrap();
            }
            let refdes_vec: Vec<&str> = line.split(" ").collect();
            let refdes_str = refdes_vec[refdes_vec.len() - 1]
                .replace("\"", "")
                .trim()
                .to_string();
            //println!("{}", refdes_str.clone());
            refdes = &refdes_str;
            //println!("{} at ({},{}) ", refdes_str, x1,y1);
            let mut in_shape: bool = true;
            while in_shape {
                while !line.contains("fp_line") {
                    //this might be where the loop ends?
                    line = content_iter.next().unwrap_or_else(|| {
                        in_shape = false;
                        "bad"
                    });

                    if line.contains("pad") {
                        in_shape = false
                    };
                }
                //now were in a shape
                let start = content_iter.next().unwrap().trim();
                let end = content_iter.next().unwrap().trim();
                while !line.contains("layer") {
                    line = content_iter.next().unwrap();
                }
                if line.contains(footprint_layer) {
                    //parse and add start and end
                    let (x, y) = parse_kicad_line_to_floats(start);
                    xs.push(x + x1);
                    ys.push(y + y1);
                    let (x, y) = parse_kicad_line_to_floats(end);
                    xs.push(x + x1);
                    ys.push(y + y1);

                    //println!("{}, {}",x + x1 ,y+ y1);
                }
            }
            //now its pin time
            //
            if !xs.is_empty() {
                xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
                ys.sort_by(|a, b| a.partial_cmp(b).unwrap());
                let mut comp_bbox = Bbox::new(xs[0], xs[xs.len() - 1], ys[0], ys[ys.len() - 1]);
                if rotation != 0 {
                    comp_bbox.rotate(rotation as f64);
                }
                let mut pin_vec: Vec<Pin> = Vec::new();
                //outer pin loop
                while !line.contains("model") && !line.starts_with("\t)") {
                    while !line.contains("pad ") {
                        line = content_iter.next().unwrap();
                        if refdes == "C4" {
                            println!("{}", line);
                        }
                        if line.contains("model") || line.starts_with("\t)") {
                            break;
                        }
                    }
                    if line.contains("model") || line.starts_with("\t)") {
                        println!("Found all pins for {}", refdes);
                        break;
                    }
                    if line.contains("pad ") {
                        line = content_iter.next().unwrap();
                        //println!("{}", line);
                        let (mut px1, mut py1) = parse_kicad_line_to_floats(line);
                        px1 += comp_bbox.centerx;
                        py1 += comp_bbox.centery;
                        line = content_iter.next().unwrap();
                        let (mut px2, mut py2) = parse_kicad_line_to_floats(line);
                        px1 -= px2 / 2.0;
                        py1 -= py2 / 2.0;

                        px2 += px1;
                        py2 += py1;
                        //let px2 =
                        //println!("{}, {}", px2, py2);
                        while !line.contains("net") && !line.starts_with("\t\t)") {
                            //println!("{}", line);
                            line = content_iter.next().unwrap();
                        }
                        let mut net = 0;
                        if !line.starts_with("\t\t)") {
                            let net_v: Vec<&str> = line.trim().split(" ").collect();
                            net = net_v[1].parse::<i32>().unwrap();
                        }
                        let pbbox = Bbox::new(px1, px2, py1, py2);
                        let pin = Pin {
                            refdes: refdes.to_string(),
                            net,
                            bbox: pbbox,
                        };
                        pin_vec.push(pin);
                    }
                } //end outer pin loop( we should have all our pins)

                //println!("{}, {}",, ys[ys.len()-1]);

                let comp: Component = Component {
                    refdes: refdes.to_string(),
                    bbox: comp_bbox,
                    rotation: 0,
                    pins: pin_vec,
                };

                //println!("{:?}", comp.pins.len());
                comp_vec.push(comp);
            }
            xs.clear();
            ys.clear();

            //Bbox::new(xs[0], x2, y1, y2)
            //in_doc = line.contains("gr_rect");
        }
    }
    Placement {
        components: comp_vec,
        placement_area: Bbox::new(0.0, 300.0, 0.0, 300.0),
    }
    //println!("{:?}", net_map);
}
