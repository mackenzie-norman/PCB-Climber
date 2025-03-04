mod plcmnt;
use plcmnt::Placement;
mod kicad_parse;
use kicad_parse::parse_file;
use clap::Parser;
mod ga;
use ga::{Individual, genetic_algorithim, generate_animation};

/*
fn synth_pl() {

    let placement_area = Bbox::new(0, 36, 0, 36);
    let pin_boxx = Bbox::new(0, 2, 0,1);
    let base_pin = Pin{refdes :"C1".to_string(), net: 0, bbox:pin_boxx };
    let mut base_pin_2 = Pin{refdes :"C1".to_string(), net: 1, bbox:pin_boxx };
    base_pin_2.move_pin(0, 3);
    let boxx = Bbox::new(0, 2, 0, 4);
    let mut c1 = Component {
        refdes: "C1".to_string(),
        bbox: boxx,
        rotation: 0,
        pins : vec![base_pin.clone(), base_pin_2.clone()]
    };
    let mut b_pin = base_pin.clone();
    let mut b_pin2 = base_pin_2.clone();
    b_pin.refdes = "C2".to_string();
    b_pin2.refdes = "C2".to_string();
    b_pin.move_pin(34, 32);
    b_pin2.move_pin(34, 32);
    b_pin2.net = 2;
    let box2 = Bbox::new(34, 36, 32, 36);
    let c2 = Component {
        refdes: "C2".to_string(),
        bbox: box2,
        rotation: 0,
        pins : vec![b_pin, b_pin2]
    };
    let mut base_pin = Pin{refdes :"C3".to_string(), net: 0, bbox:pin_boxx };
    base_pin.move_pin(11, 5);
    let mut base_pin_2 = Pin{refdes :"C3".to_string(), net: 1, bbox:pin_boxx };
    base_pin_2.move_pin(7, 5);
    let mut base_pin_3 = Pin{refdes :"C3".to_string(), net: 2, bbox:pin_boxx };
    base_pin_3.move_pin(4, 0);
    let box3 = Bbox::new(4, 13, 0, 6);
    let mut c3 = Component {
        refdes: "C3".to_string(),
        bbox: box3,
        rotation: 0,
        pins : vec![base_pin.clone(), base_pin_2.clone(), base_pin_3]
    };
    let mut c4 = c2.clone();
    c4.set_refdes("C4".to_string());
    let mut c5 = c1.clone();
    c5.set_refdes("C5".to_string());
    c1.move_comp(6, 6);
    //c2.move_comp(6, 6);
    c4.move_comp(0, -6);
    c5.move_comp(15, 2);
    c3.move_comp(6, 6);
    //c1.move_comp( 10, 11);
    //for i in 1..2000{ c1.rotate_comp(90);};
    c3.rotate_comp(180);
    //c3.rotate_comp(90);
    let comps: Vec<Component> = vec![c1, c2,c3,c4, c5];

    let pl = Placement {
        components: comps,
        placement_area,
    };
}
*/
fn tester(pl: Placement) {
    let pl_2 = pl.clone();
    let id2 = Individual::new(pl_2);
    id2.plot("0.png", &pl.net_map);
    let gen_mult = 1;
    let test_cases: Vec<(u32, u32)> = vec![
        (10, 10000 * gen_mult),
        (20, 5000 * gen_mult),
        (50, 2000 * gen_mult),
        (100, 1000 * gen_mult),
        (200, 500 * gen_mult),
        (500, 200 * gen_mult),
    ];
    for i in test_cases {
        genetic_algorithim(pl.clone(), i.0, i.1, true);
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the .kicad_pcb file to use
    #[arg(short, long, default_value_t = ("../arduino_kicad/arduino UNO.kicad_pcb".to_string()) )]
    file: String,

    /// Number of generations
    #[arg(short, long, default_value_t = 1000)]
    generations: u32,
    /// How many individuals are in our popuation
    #[arg(short, long, default_value_t = 100)]
    population_size: u32,
    ///Run the testing function on our file (will overwrite gen/pop)
    #[arg(short, long, default_value_t = false)]
    test: bool,
    ///Selection Type (ev or elitist)
    #[arg(short, long, default_value_t = false)]
    selection: bool,
    ///Generate an animation 
    #[arg(short, long, default_value_t = false)]
    animate: bool,

}
fn main() {
    let args = Args::parse();
    let mut pl2: Placement = parse_file(&args.file);
    pl2.shift_placement(0.0, 0.0);
    
    let test = args.test;
    let anim = args.animate;
    if test {
        tester(pl2);
    } else {
        if ! anim{

            let _scores =genetic_algorithim(pl2, args.population_size, args.generations, true);
        }else{
            let _ = generate_animation(pl2);
        }
        //println!("{:?}", scores);
    }
}
