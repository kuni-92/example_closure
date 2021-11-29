use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculation slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_value: u32) {
    let expensive_result = simulated_expensive_calculation(intensity);

    if intensity < 25 {
         println!(
            "Today, do {} push up!",
            expensive_result
        );
        println!(
            "Next, do {} shitups!",
            expensive_result
        );
    } else {
        if random_value == 3 {
            println!("Take a break today!");
        } else {
            println!(
                "Today, ran for{} minutes!",
                expensive_result
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number,
    );
}
