// 1. Define a trait named `Bite`
//
// Define a single required method, `fn bite(self: &mut Self)`.  We will call this method when we
// want to bite something.  Once this trait is defined, you should be able to run the program with
// `cargo run` without any errors.
//
//  trait Bite...
trait Bite{
    fn bite(self: &mut Self);
}

// 2. Now create a struct named Grapes with a field that tracks how many grapes are left.  If you
// need a hint, look at how it was done for Carrot at the bottom of this file (you should probably
// use a different field, though).
//


// 3. Implement Bite for Grapes.  When you bite a Grapes, subtract 1 from how many grapes are left.
// If you need a hint, look at how it was done for Carrot at the bottom of this file.
//


fn main() {
    // Once you finish #1 above, this part should work.
    let mut carrot = Carrot { percent_left: 100.0 };
    carrot.bite();
    println!("I take a bite: {:?}", carrot);

    // 4. Uncomment and adjust the code below to match how you defined your

    let mut grapes = Grapes { grapes_left: 100 };
    grapes.bite();
    println!("Eat a grape: {:?}", grapes);

    // Challenge: Uncomment the code below. Create a generic `bunny_nibbles`
    // function that:
    // - takes a mutable reference to any type that implements Bite
    // - calls `.bite()` several times
    // Hint: Define the generic type between the function name and open paren:
    //       fn function_name<T: Bite>(...)

    bunny_nibbles(&mut carrot);
    bunny_nibbles(&mut grapes);
    println!("Bunny nibbles for awhile: {:?}", carrot);
    println!("Bunny eats for a while : {:#?}", grapes);
}

fn bunny_nibbles<T: Bite>(item: &mut T){
    for n in 1..=10{
        item.bite();
    }
}

#[derive(Debug)] // This enables using the debugging format string "{:?}"
struct Carrot {
    percent_left: f32,
}

#[derive(Debug)] // include this line right before your struct definition
struct Grapes{
    grapes_left:i32,
}


impl Bite for Carrot {
    fn bite(self: &mut Self) {
        // Eat 20% of the remaining carrot. It may take awhile to eat it all...
        self.percent_left *= 0.8;
    }
}
impl Bite for Grapes{
    fn bite(self: &mut Self) {
        self.grapes_left -=1;
    }
}
