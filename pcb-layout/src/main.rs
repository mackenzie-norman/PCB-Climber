use std::collections::BTreeMap;
mod plcmnt;
use plcmnt::{Bbox, Component, Pin, Placement};
mod kicad_parse;
use clap::Parser;
use kicad_parse::parse_file;
mod ga;
use ga::{elitist_selection, ev_selection, generate_animation, genetic_algorithim, Individual};
mod sa;
use sa::{log_cool, quick_sa};
/// # Generates a really simple placement
///
///
///
///
fn gen_synth_pl() -> Placement {
    let placement_area = Bbox::new(0.0, 36.0, 0.0, 36.0);
    let pin_boxx = Bbox::new(0.0, 2.0, 0.0, 1.0);
    let base_pin = Pin {
        refdes: "C1".to_string(),
        net: 0,
        bbox: pin_boxx,
    };
    let mut base_pin_2 = Pin {
        refdes: "C1".to_string(),
        net: 1,
        bbox: pin_boxx,
    };
    base_pin_2.move_pin(0.0, 3.0);
    let boxx = Bbox::new(0.0, 2.0, 0.0, 4.0);
    let mut c1 = Component {
        refdes: "C1".to_string(),
        bbox: boxx,
        rotation: 0,
        pins: vec![base_pin.clone(), base_pin_2.clone()],
    };
    let mut b_pin = base_pin.clone();
    let mut b_pin2 = base_pin_2.clone();
    b_pin.refdes = "C2".to_string();
    b_pin2.refdes = "C2".to_string();
    b_pin.move_pin(34.0, 32.0);
    b_pin2.move_pin(34.0, 32.0);
    b_pin2.net = 2;
    let box2 = Bbox::new(34.0, 36.0, 32.0, 36.0);
    let c2 = Component {
        refdes: "C2".to_string(),
        bbox: box2,
        rotation: 0,
        pins: vec![b_pin, b_pin2],
    };
    let mut base_pin = Pin {
        refdes: "C3".to_string(),
        net: 0,
        bbox: pin_boxx,
    };
    base_pin.move_pin(11.0, 5.0);
    let mut base_pin_2 = Pin {
        refdes: "C3".to_string(),
        net: 1,
        bbox: pin_boxx,
    };
    base_pin_2.move_pin(7.0, 5.0);
    let mut base_pin_3 = Pin {
        refdes: "C3".to_string(),
        net: 2,
        bbox: pin_boxx,
    };
    base_pin_3.move_pin(4.0, 0.0);
    let box3 = Bbox::new(4.0, 13.0, 0.0, 6.0);
    let mut c3 = Component {
        refdes: "C3".to_string(),
        bbox: box3,
        rotation: 0,
        pins: vec![base_pin.clone(), base_pin_2.clone(), base_pin_3],
    };
    let mut c4 = c2.clone();
    c4.set_refdes("C4".to_string());
    let mut c5 = c1.clone();
    c5.set_refdes("C5".to_string());
    c1.move_comp(6.0, 6.0);
    //c2.move_comp(6, 6);
    c4.move_comp(0.0, -6.0);
    c5.move_comp(15.0, 2.0);
    c3.move_comp(6.0, 6.0);
    //c1.move_comp( 10, 11);
    //for i in 1..2000{ c1.rotate_comp(90);};
    c3.rotate_comp(180);
    //c3.rotate_comp(90);
    let comps: Vec<Component> = vec![c1, c2, c3, c4, c5];
    let mut net_map: BTreeMap<i32, String> = BTreeMap::new();
    net_map.insert(0, "GND".to_string());
    net_map.insert(1, "5V+".to_string());
    net_map.insert(2, "5V+".to_string());

    Placement {
        components: comps,
        placement_area,
        net_map,
    }
}

///# Runs our simple test suite
/// This will output the time, the score, and save a picture
/// ## Test Cases
/// - Population: 10, Generation Count: 10000
/// - Population: 20, Generation Count: 5000
/// - Population: 50, Generation Count: 2000
/// - Population: 100, Generation Count: 1000
/// - Population: 200, Generation Count: 500
/// - Population: 500, Generation Count: 200
///
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
        genetic_algorithim(pl.clone(), i.0, i.1, true, ev_selection, 1);
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the .kicad_pcb file to use. Use synthetic to generate a fake board
    #[arg(short, long, default_value_t = ("../arduino_kicad/arduino UNO.kicad_pcb").to_string() )]
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
    ///Number of threads
    #[arg(short, long, default_value_t = 1)]
    threads: u32,
}
fn main() {
    let args = Args::parse();
    //Parse Our Kicad and put it at 0,0
    // (We can always move this back)
    let mut pl2: Placement;
    if &args.file == "synthetic" {
        pl2 = gen_synth_pl();
    } else {
        pl2 = parse_file(&args.file);
    }
    pl2.shift_placement(0.0, 0.0);
    //for SA
    let sa_pl = pl2.clone();
    let test = args.test;
    let anim = args.animate;
    let sel_type = args.selection;
    let mut selection_algo: fn(&mut Vec<Individual>) = ev_selection;
    if sel_type {
        selection_algo = elitist_selection;
    }
    if test {
        tester(pl2);
    } else if !anim {
        let _scores = genetic_algorithim(
            pl2,
            args.population_size,
            args.generations,
            true,
            selection_algo,
            args.threads,
        );
    } else {
        let _ = generate_animation(pl2);
    }
    let clone_sa = sa_pl.clone();
    let id2 = Individual::new(sa_pl);
    let id3 = quick_sa(
        id2,
        log_cool,
        (args.generations * args.population_size)
            .try_into()
            .unwrap(),
        true,
    );

    id3.plot("test-sa.png", &clone_sa.net_map);
}
