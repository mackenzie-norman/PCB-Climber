use crate::plcmnt;
use plcmnt::{Bbox, Component, Pin, Placement};
use std::collections::BTreeMap;

use std::fs;
///Getting the two (usually x,y floats from a kicad placement)
fn parse_kicad_line_to_floats(passed_str: &str) -> Option<(f64, f64)> {
    let x_y_str = passed_str.trim().replace(")", "");
    let x_y_vec: Vec<&str> = x_y_str.split(" ").collect();
    if x_y_vec.len() < 3 {
        //println!("{:?}", x_y_vec);
        return None;
    }

    Some((
        x_y_vec[1].parse::<f64>().unwrap(),
        x_y_vec[2].parse::<f64>().unwrap(),
    ))
}
/// # Responsible for parsing a kicad
/// Takes a filename as an &str
///
///
pub fn parse_file(file_path: &str) -> Placement {
    // --snip--
    //let file_path = "..\\demo\\demo.kicad_pcb";
    //let file_path = "..\\demo\\layout1.kicad_pcb";
    //let file_path = "..\\BeagleBone_Black.unrouted.kicad_pcb";
    //let file_path = "..\\arduino_kicad\\Arduino UNO.kicad_pcb";
    //println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut net_map: BTreeMap<i32, String> = BTreeMap::new();
    let mut comp_vec: Vec<Component> = Vec::new();
    //The layer used to calc the area of the foot print
    let pcb_layer = "Edge.Cuts";
    let footprint_layer = "CrtYd";
    let mut refdes: &String;
    //= "some";
    let mut x1: f64;
    //= 0.0;
    let mut y1: f64;
    let mut rotation: i32;
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

        if line.starts_with("\t(net ") && line.trim_end().ends_with("\")") {
            let words: Vec<&str> = line.split(" ").collect();
            let net_idx = words[1].parse::<i32>().unwrap();
            let net_name = words[2]
                .trim_end()
                .strip_suffix(|_: char| true)
                .unwrap()
                .replace("\"", "");

            net_map.insert(net_idx, net_name);
        }
        if line.starts_with("\t(footprint ") {
            let _ = content_iter.next().unwrap();
            let _ = content_iter.next().unwrap();
            let x_y_str = content_iter.next().unwrap().trim_end().replace(")", "");
            let x_y_vec: Vec<&str> = x_y_str.split(" ").collect();

            x1 = x_y_vec[1].parse::<f64>().unwrap();
            y1 = x_y_vec[2].parse::<f64>().unwrap();
            if x_y_vec.len() > 3 {
                rotation = x_y_vec[3].parse::<i32>().unwrap_or(0)
            } else {
                rotation = 0
            }

            while !line.contains("Reference") {
                line = content_iter.next().unwrap();
            }
            let refdes_vec: Vec<&str> = line.split(" ").collect();
            let refdes_str = refdes_vec[refdes_vec.len() - 1]
                .replace("\"", "")
                .trim()
                .to_string();

            refdes = &refdes_str;

            let mut in_shape: bool = true;
            while in_shape {
                while !line.contains("fp_") {
                    //this might be where the loop ends?
                    line = content_iter.next().unwrap_or_else(|| {
                        in_shape = false;
                        "bad"
                    });

                    if line.contains("pad") {
                        in_shape = false
                    };
                    if !in_shape {
                        break;
                    }
                }

                //now were in a shape
                let start = content_iter.next().unwrap().trim();
                let end = content_iter.next().unwrap().trim();
                while !line.contains("layer") {
                    line = content_iter.next().unwrap();
                    if line.contains("pad") {
                        in_shape = false;
                        break;
                    };
                }
                if line.contains(footprint_layer) {
                    //parse and add start and end
                    let opt = parse_kicad_line_to_floats(start);
                    if let Some(tple) = opt {
                        let (x, y) = tple;
                        xs.push(x + x1);
                        ys.push(y + y1);
                    }
                    let opt = parse_kicad_line_to_floats(end);
                    if let Some(tple) = opt {
                        let (x, y) = tple;
                        xs.push(x + x1);
                        ys.push(y + y1);
                    }
                }
            }
            //now its pin time
            //
            if !xs.is_empty() {
                xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
                ys.sort_by(|a, b| a.partial_cmp(b).unwrap());
                let comp_bbox = Bbox::new(xs[0], xs[xs.len() - 1], ys[0], ys[ys.len() - 1]);
                let mut pin_vec: Vec<Pin> = Vec::new();

                //outer pin loop
                while !line.contains("model") && !line.starts_with("\t)") {
                    while !line.contains("pad ") {
                        line = content_iter.next().unwrap();

                        if line.contains("model") || line.starts_with("\t)") {
                            break;
                        }
                    }
                    if line.contains("model") || line.starts_with("\t)") {
                        break;
                    }
                    if line.contains("pad ") {
                        line = content_iter.next().unwrap();

                        let (mut px1, mut py1) = parse_kicad_line_to_floats(line).unwrap();

                        px1 += x1;
                        py1 += y1;
                        line = content_iter.next().unwrap();
                        let (mut px2, mut py2) = parse_kicad_line_to_floats(line).unwrap();
                        px1 -= px2 / 2.0;
                        py1 -= py2 / 2.0;

                        px2 += px1;
                        py2 += py1;

                        while !line.contains("net") && !line.starts_with("\t\t)") {
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

                let mut comp: Component = Component {
                    refdes: refdes.to_string(),
                    bbox: comp_bbox,
                    rotation: 0,
                    pins: pin_vec,
                };
                if rotation != 0 {
                    comp.rotate_comp(rotation);
                }

                comp_vec.push(comp);
            }
            xs.clear();
            ys.clear();
        }

        if line.contains("(gr_") {
            let start = content_iter.next().unwrap().trim();
            let end = content_iter.next().unwrap().trim();

            while !line.contains("layer") {
                line = content_iter.next().unwrap();
            }
            if line.contains(pcb_layer) {
                let opt = parse_kicad_line_to_floats(start);
                if let Some(tple) = opt {
                    let (x, y) = tple;
                    xs.push(x);
                    ys.push(y);
                }
                let opt = parse_kicad_line_to_floats(end);
                if let Some(tple) = opt {
                    let (x, y) = tple;
                    xs.push(x);
                    ys.push(y);
                }
            }
        }
    }

    let mut pl_area = Bbox::new(0.0, 300.0, 0.0, 300.0);
    if !xs.is_empty() {
        xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        ys.sort_by(|a, b| a.partial_cmp(b).unwrap());
        pl_area = Bbox::new(xs[0], xs[xs.len() - 1], ys[0], ys[ys.len() - 1]);
    }

    Placement {
        components: comp_vec,
        placement_area: pl_area,
        net_map,
    }
}
