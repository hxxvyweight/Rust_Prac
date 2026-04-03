use std::io;

fn main() {
    

    //Welcome input and User Prompts//

    println!("Welcome!");

    let mut user_input = String::new();
    
    io::stdin().read_line(&mut user_input).expect("Failed to read");

    println!("Basic Text Game!\n");
    
    fn prompt_user() {
        println!("Press ENTER to continue");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
    }
   
    loop {
        prompt_user();
           break;
    }
     
    //Basic Level, Stage, Stats, and Available Class Names//
    const MAP: &str = "Map";

    println!("Welcome to {}\n", MAP);
   
    prompt_user();
   
    let stage: [&str; 3] = ["Stage 1\n","Stage 2\n","Stage 3\n"]; 
    
    println!("{}", stage[0]);
   
    prompt_user();
  
    let _classes: [&str; 3] = ["Warrior\n", "Mage\n", "Rogue\n"];

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
    
    //Basic Encounter System for Combat//


    fn encounter() {
        
        let mut enemy_hp: i32 = 250;
        let enemy_dmg: i32 = 25;
        let mut user_hp: i32 = 500;
        let user_dmg: i32 = 50;


        loop {  
            
            println!("An enemy stands before you!");

            //Reading for User Input and assigning to variable//
            
            let mut user_choice:String = String::new();

            io::stdin()
                .read_line(&mut user_choice)
                .expect("Failed to read_line");
                let user_choice = user_choice.trim();    
                println!("Attack or defend");
                //Getting Input and Trimming Whitespace// 
                
                //Attack and Defend Prompt temporarily//
                
                if user_choice == "Attack"{
                    println!("Success");
                    enemy_hp -= user_dmg;
                    println!("User HP: {} | Enemy HP: {}", user_hp, enemy_hp);
                    continue;
                    
                }else if enemy_hp <= 0 {
                    println!("Enemy defeated! {}", enemy_hp);
                    break;
                }else if user_choice == "Defend" {
                    user_hp -= enemy_dmg;
                    println!("You lost {}\n\nUser HP: {}", enemy_dmg, user_hp);
                    continue;
                
                 //End of Encounter// 
        }
                  
        }
    }

    encounter();


    println!("{}, {}, {}", stats.strength, stats.magic, stats.stamina);

}
