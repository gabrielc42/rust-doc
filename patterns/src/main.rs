fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "25".parse();

    if let Some(color) = favorite_color {
        println!("Your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Purple as the background color");
        } else {
            println!("Orange as the background color");
        }
    } else {
        println!("Blue as the background color");
    }
}
