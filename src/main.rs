use std::thread;
use std::time::Duration;

struct Cacher<T>                              // idhar ham struct bana rahe hai taaki ham closur jo ek expensive function hai usko 
where                                         // baar bar call na karna pade and usko use kar sake multiple time (memorization or lazy evalution)          
    T: Fn(u32) -> u32,                        // har clouser ka type uniuqe(alag) hota hai tab bhi jab unka signature same hoo    
{                                             // issliye hame genric and trait ka use karna padega inko use karne ke liye(trait = complusory to use if defined)
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,                         // function bhi saare trait use kar sakte hai fn trait ke closur ki jagah lekin conditon(agar capture na karna ho value ko jo result mai anna)
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,                     // closur ki value none(kuch nhi rahega) before after use of closure value jo bhi hoga jo nikalega
            value: None,                     // next time se woh value save rahega (dubra excute karne ki jarurt nhi padega)
        }
    }

    fn value(&mut self, arg: u32) -> u32 {  // idhar yeh closure ko dubra call karne se phele value method ko call karega or check karega ki yeh call ho rakha hai ki nhi
        match self.value {                  // closur ki direct value safe karne se accha yeh ek instance mai uski value define karke sae kar deta hai  
            Some(v) => v,               // check karta hai ki expensive function bina matlab baar baar call na hua hoo
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}
