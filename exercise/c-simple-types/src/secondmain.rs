#![allow(unused_variables, unused_mut, dead_code)]
enum Shot{
    Bullseye,
    Hit(f64),
    Miss,
}

impl Shot{

    fn points(&self) -> f64{
    match self {
        Shot::Bullseye => 5,
        // guards
        Shot::Hit(value) if value < 3.0=>2,
        Shot::Hit(value) => 1,
        Shot::Miss => 0,
    }
}

fn main(){
    let coordinates:Vec<Coords> = get_arrow_coords(5);;
    let mut shotvector:Vec<Shot> = Vector::new()

    for cor in &coordinates{
        cor.print_description();
        let appendi = cor.distn();
        match appendi {
            appendi < 1 => shotvector.push(Shot::Bullseye);
        }
        
    }
}

struct Coords{
    x:f64,
    y:f64,
}

impl Coords{
    fn distn(&self)->f64{
        (&self.x.powf(2.0)+&self.y.powf(2.0)).sqrt()
    }
    fn print_description(&self){
        print!("{:?}", &self);
        println!("Away of {:?}", &self.distn());
    }

}

fn get_arrow_coords(num:u32) -> Vector<Coords>{
    let mut vec:Vector<Coords> = Vector::new();
    for n in 0..num{
        let coordo = {
            x : rand::random::f64(),
            y: rand::random::f64(),
        };
        vec.push(cordo);
    }
    vec
}
