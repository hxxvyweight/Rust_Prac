use std::io;

fn return_outcome(success: bool, level: &mut u32, current_xp: &mut u32, required_xp: &mut u32) {
        
    let mut encountered: bool = true;
       
    while encountered {
            
        if success {
            *current_xp += 25;
            *required_xp -= 25;
            println!("Level: {}, Current XP: {}, Required: {}", level, current_xp, required_xp);
            encountered = false;
        } else {
            println!("Try again!");
        }
    }
}

fn encounter() -> bool {
        
    let mut enemy_hp: i32 = 250;
    let enemy_dmg: i32 = 25;
    let mut user_hp: i32 = 250;
    let user_dmg: i32 = 50;
    let mut killcount: u32 = 0;


    loop {  
            
        let mut user_choice:String = String::new();
        user_choice.clear();
       
        println!("An enemy stands before you!\n");

            //Reading for User Input and assigning to variable//
            
        println!("Attack or defend\n");
           

        io::stdin()
            .read_line(&mut user_choice)
            .expect("Failed to read_line");
            let user_choice = user_choice.trim().to_lowercase();    
                //Getting Input and Trimming Whitespace// 

                
                
            if user_choice == "attack"{
                enemy_hp -= user_dmg;
                println!("\nUser HP: {} | Enemy HP: {}\n", user_hp, enemy_hp);

                if enemy_hp <= 0 {
                    println!("Enemy defeated! {}\n", enemy_hp);
                    killcount += 1;
                    println!("Enemies Defeated: {}", killcount);
                    
                    return true;
                }
                
            }else if user_choice == "defend" {
                user_hp -= enemy_dmg;
                println!("\nUser HP: {} | Enemy HP: {}\n", user_hp, enemy_hp);
                
                if user_hp <= 0 {
                    println!("Game Over...\n");
                    println!("You Defeated {} Enemies!!!\nCongratulations", killcount); 
                    return true;
                }
            }
    }
}


fn prompt_user() {
    println!("Press ENTER to continue");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

fn main() {
    

    //Welcome input and User Prompts//

    println!("Welcome!");

    let mut user_input = String::new();
    
    io::stdin().read_line(&mut user_input).expect("Failed to read");

    println!("Basic Text Game!\n");
    
   
    loop {
        prompt_user();
           break;
    }
     
    //Basic Level, Stage, and Available Class Names//
    const MAP: &str = "MAP";

    println!("Welcome to {}\n", MAP);
   
    prompt_user();
   
    let stage: [&str; 3] = ["Stage 1\n","Stage 2\n","Stage 3\n"]; 
    
    println!("{}", stage[0]);
   
    prompt_user();
  
    let _classes: [&str; 3] = ["Warrior\n", "Mage\n", "Rogue\n"];
    
    //Character Stats and XP//
    struct Stats {
        strength: i32,
        magic: i32,
        stamina: i32,
    }
    
    let stats: Stats = Stats {
        strength: 1,
        magic: 1,
        stamina: 1,
    };
        
    struct XP {
        level: u32,
        current_xp: u32,
        required_xp: u32,
    }

    let mut xp_pool: XP = XP {
        level: 1,
        current_xp: 0,
        required_xp: 200,
    };
    
    let mut action = prompt_user();
    if action == "fight" {
        let success = encounter();
    }
    
    return_outcome(success, &mut xp_pool.level, &mut xp_pool.current_xp, &mut xp_pool.required_xp);
   
    println!("- Strength: {}\n- Magic: {}\n- Stamina: {}\n", stats.strength, stats.magic, stats.stamina);

}

