use std::io;
use self::operations::todos::TodoList;
use self::operations::error::TodoError;


mod operations;


fn main() {

    let mut todo_list = TodoList::new();
    println!("\nWelcome to this To-Do List made in Rust!!!\n");

    println!(" ____________        __________        _____________           __________    ");
    println!("(____   _____)     / ________  \\      |  _________  \\        / ________  \\  ");
    println!("     |  |         / /         \\ \\     | |         \\  \\      / /         \\ \\");
    println!("     |  |        | |           | |    | |          |  |    | |           | | ");
    println!("     |  |        | |           | |    | |          |  |    | |           | | ");
    println!("     |  |        | |           | |    | |          |  |    | |           | | ");
    println!("     |  |         \\ \\_________/ /     | |_________/  /      \\ \\_________/ /");
    println!("     (__)          \\___________/      |_____________/        \\___________/  ");
    println!("\n");

    
    loop { // loops untill the user selects the exit option.
        println!("Please select the action you would like to perform.");
        println!("1) Add a new To-Do.");
        println!("2) Mark a To-do as complete/incomplete.");
        println!("3) Delete a To-do.");
        println!("4) Print the current To-do list");
        println!("5) Exit the program");  
    
        println!("\nEnter your choice:");
       
        let mut ch = String::new();
        io::stdin()
            .read_line(&mut ch)
            .expect("Unable to capture input");

        let ch: u32 = match ch.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("\nError!!\n{}\n", TodoError::InvalidInput(e).err());
                continue;
            },
        };
    
        println!("Your choice is: {ch}");

        match ch {
            1 => {
                println!("Enter the name of the new task: ");
                let mut task = String::new();
                io::stdin()
                    .read_line(&mut task)
                    .expect("Unable to read input.");

                let task = task.trim().to_string();
                if task.is_empty() {
                    println!("\nError!!\nNoInput::{{Task not entered}}\n");
                } else {
                    todo_list.insert(task);
                }
            },
            2 => {

                    if let Err(e) = todo_list.mark(){
                        println!("Error!!\n{}\n", e.err());
                    };
            },

            3 =>{        
                    if let Err(e) = todo_list.delete() {
                        println!("Error!!\n{}\n", e.err());
                    };
            },

            4 => if let Err(e) = todo_list.print_list() {
                println!("Error!!\n{}\n", e.err()); 
            },
            5 => {
                println!("Exiting app!!!\n");
                return
            },
            _ => println!("Please Enter the correct choice"),
        }    
    }
                                    
}
