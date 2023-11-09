use oop;
fn main() {
    demo_averagedcollection();
}

fn demo_averagedcollection () {
    let mut ac = oop::AveragedCollection::new();
    ac.add(5);
    println!("average is {}", ac.get_average());
    ac.add(6);
    println!("average is {}", ac.get_average());
    ac.add(10);
    println!("average is {}", ac.get_average());
    ac.remove();
    println!("average is {}", ac.get_average());
    ac.remove();
    println!("average is {}", ac.get_average());
}

