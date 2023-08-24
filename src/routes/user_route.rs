use rand::random;

use crate::models::user_model::print_user_model as baboon;
use super::health_route::print_health_route;

// routes/user_route.rs
pub fn print_user_route() {
    baboon();
    let random_number: u8 = random();
    println!("{}", random_number);
    print!("calling health route from user_route");
    print_health_route(); // super refers to the parent


    println!("user route");
}
