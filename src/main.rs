use std::io;

// destructure the tuple later to edit its contents
struct Task {
    task_name:String, // eg. do math homework
    task_description:String, // eg. submit them via managebac after completing them via linkedin
    task_deadline:[i32; 3],
    task_urgency:UrgencyLevel,
}

enum UrgencyLevel {
    Low,
    Medium,
    High,
}

fn main() {
    /*let mut user_input:String = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    println!("aw yeah baby {}", user_input);*/

    let mut storage_array:Vec<(String, String, [i32;3], UrgencyLevel)> = vec![];

    loop {
        
        // break condition
        println!("Press [E] to exit");
        let mut userinput_exit:String = String::new();
        io::stdin().read_line(&mut userinput_exit).expect("Failed to read line");
        if userinput_exit == "e" {
            break
        }

        // task name
        println!("Enter task name: ");
        let mut userinput_task_name:String = String::new();
        io::stdin().read_line(&mut userinput_task_name).expect("Failed to read line");
        
        // task description
        println!("Enter task description: ");
        let mut userinput_task_description:String = String::new();
        io::stdin().read_line(&mut userinput_task_description).expect("Failed to read line");

        // task deadline -> parsed using destructuring

        println!("Enter task deadline year [20XX]: ");
        let mut userinput_task_deadline_year:String = String::new();
        io::stdin().read_line(&mut userinput_task_deadline_year).expect("Failed to read line");
        let userinput_task_deadline_year_int:i32 = userinput_task_deadline_year.parse().unwrap();

        println!("Enter task deadline month [1-12]: ");
        let mut userinput_task_deadline_month:String = String::new();
        io::stdin().read_line(&mut userinput_task_deadline_month).expect("Failed to read line");
        let userinput_task_deadline_month_int:i32 = userinput_task_deadline_month.parse().unwrap();

        println!("Enter task deadline day [1-31]: ");
        let mut userinput_task_deadline_day:String = String::new();
        io::stdin().read_line(&mut userinput_task_deadline_day).expect("Failed to read line");
        let userinput_task_deadline_day_int:i32 = userinput_task_deadline_day.parse().unwrap();

        let userinput_task_deadline_formatted:[i32; 3] = [userinput_task_deadline_day_int, userinput_task_deadline_month_int, userinput_task_deadline_year_int];

        // task urgency -> parsed into an enum
        println!("Enter task urgency (L/M/H): ");
        let mut userinput_task_urgency_string:String = String::new();
        io::stdin().read_line(&mut userinput_task_urgency_string).expect("Failed to read line");
        let userinput_task_urgency_stringliteral:&str = &userinput_task_urgency_string[..];
        let userinput_task_urgency:UrgencyLevel;
        match userinput_task_urgency_stringliteral {
            "l" => {
                userinput_task_urgency = UrgencyLevel::Low;
            }, 
            "m" => {
                userinput_task_urgency = UrgencyLevel::Medium;
            },
            "h" => {
                userinput_task_urgency = UrgencyLevel::High;
            },
            _ => {
                println!("Defaulting to low task urgency!");
                userinput_task_urgency = UrgencyLevel::Low;
            },
        }
        
        storage_array.push((userinput_task_name, userinput_task_description, userinput_task_deadline_formatted, userinput_task_urgency));
    }
    for tupleset in storage_array {
        println!("{}", tupleset.1);
    }
}
