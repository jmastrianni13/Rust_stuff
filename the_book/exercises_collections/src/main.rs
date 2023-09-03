use exercises_collections::ex_1;
use exercises_collections::ex_2;
use exercises_collections::ex_3;

fn main() {
    demo_ex_1();   
    demo_ex_2();
    demo_ex_3();
}

fn demo_ex_1() {

    let numbers = vec![5, 3, 1, 4, 2, 6, 4, 5, 5, 5, 5, 2, 3, 1, 99];
    let median = ex_1::get_median(&numbers);
    let mode = ex_1::get_mode(&numbers);
    println!("numbers are                 {:?}", numbers);
    println!("median is {median}");
    println!("mode is {mode}");
    println!("numbers should be untouched {:?}", numbers);

   
}

fn demo_ex_2() {
    ex_2::get_pig_latin("apple");
    ex_2::get_pig_latin("rust");
}

fn demo_ex_3() {
    ex_3::emp_app();
}

