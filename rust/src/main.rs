use std::io;

fn main() {
    

    //Welcome input//

    println!("Welcome!");

    let mut user_input = String::new();
    
    io::stdin().read_line(&mut user_input).expect("Failed to read");

    println!("Basic Text Game!\n");
    
    std::mem::drop(user_input);

    fn prompt_user() {
        println!("Press ENTER to continue");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
    }
   
    loop {
        prompt_user();
           break;
    }
     

    const MAP: &str = "Map";

    println!("Welcome to {}\n", MAP);
   
    prompt_user();
   
    let stage: [&str; 3] = ["Stage 1","Stage 2","Stage 3"]; 
    
    println!("{}", stage[0]);
   
    prompt_user();
  
    let _classes: [&str; 3] = ["Warrior", "Mage", "Rogue"];

    struct Stats {
        strength: i32,
        magic: i32,
        stamina: i32,
    }
    
    let mut stats = Stats {
        strength: 0,
        magic: 0,
        stamina: 0,
    };
    stats.strength += 1; 
    println!("Strength {}, Stamina: {}, Magic: {}", stats.strength, stats.magic, stats.stamina);

    




}

