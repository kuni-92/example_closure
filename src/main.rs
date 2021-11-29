use std::thread;
use std::time::Duration;

struct Casher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Casher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Casher<T> {
        Casher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

fn generate_workout(intensity: u32, random_value: u32) {
    let mut expensive_result = Casher::new(|num| {
        println!("calculation slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
         println!(
            "Today, do {} push up!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} shitups!",
            expensive_result.value(intensity)
        );
    } else {
        if random_value == 3 {
            println!("Take a break today!");
        } else {
            println!(
                "Today, ran for{} minutes!",
                expensive_result.value(intensity)
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
