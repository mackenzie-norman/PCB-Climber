struct Bbox{
    x1:i32,
    x2:i32,
    y1:i32,
    y2:i32,
    centerx: i32,
    centery: i32
}

impl Bbox{
    fn new(x1:i32 ,x2:i32 ,y1:i32 ,y2:i32 ) -> Bbox{
        Bbox { x1: x1, x2: x2, y1: y1, y2: y2, centerx: (x1-x2).abs() / 2, centery: (y1-y2).abs()/2 }
    }
}
struct Placement{
    components : Vec<Component>,

    
}
struct Component{
    refdes : String,
    bbox: Bbox,
    rotation: i32,
}
impl Component{
    fn string(&self) -> String{
        return  self.refdes.clone() + " at (" + &self.bbox.centerx.to_string()  +"," + &self.bbox.centery.to_string() + ")";
    }
    fn move_comp(&mut self,x:i32 ,y:i32){
        self.bbox.x1 += x;
        self.bbox.y1 += y;
        self.bbox.x2 += x;
        self.bbox.y2 += y;
    }
    fn rotate_comp(& mut self, delta: i32){
        self.rotation += delta;
    }

}
fn main() {
    println!("Hello, world!");
    //let mut boxx= Bbox::new(0,2,0,4);
    //let mut c1 = Component{refdes: "C1".to_string(), bbox:boxx, rotation:0};
    //println!("{}",(c1.string()));
    //c1.move_comp( 10, 11);
    //c1.rotate_comp(90);
    
    //println!("{}",(c1.string()))
    let brd_path = std::path::Path::new("../BeagleBone_Black.unrouted.kicad_pcb");
    let read_result = kicad_parse_gen::read_layout(brd_path);
    let brd = read_result.unwrap();
    ()
    
    
}
