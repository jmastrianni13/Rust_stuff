fn main() {
    let number = 7;
    if number < 5 {
        println!("condition is true, consequent expression is evaluated")
    } else {
        println!("condition is false, alternative expression is evaluated")
    }

    let condition = true;
    let t_cond = if condition { 5 } else { 50 };

    let condition = false;
    let f_cond = if condition { 5 } else { 50 };

    println!("t_cond = {t_cond}");
    println!("f_cond = {f_cond}");
}
