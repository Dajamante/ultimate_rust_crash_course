fn main() {
   let x = do_stuff(2.0, 3.0);
   print!("{:?}",x);
}

fn do_stuff(qty: f64, oz: f64) -> f64 {
   println!("{} {} -oz nämennämenämen()!", qty, oz);
   //tail expression: a return without ;
   qty * oz
}
