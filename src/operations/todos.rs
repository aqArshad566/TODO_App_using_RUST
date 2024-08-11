use crate::TodoError;
use std::io;

//Holds the details of a single to-do task
#[derive(Debug, Clone)]
enum Status{
    Pending,
    Finished,
}

#[derive(Debug, Clone)]
pub struct Todo {
    task: String,
    status: Status,
}

#[derive(Debug, Clone)]
pub struct TodoList {
    pub todos: Vec<Todo>
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList { todos: vec![] }
    }

    pub fn insert(&mut self, task: String) {
        let todo = Todo {
            task,
            status: Status::Pending,
        };

        self.todos.push(todo);

        println!("\nNew Todo created!!\n{:?}\n", self.todos.last().unwrap());
    }

    pub fn mark(&mut self) -> Result<(),TodoError> {
        
        self.print_list()?; // Prints the list or propagates the empty list error.

        let todos = &mut self.todos;

        println!("\nEnter the task number to be marked");

        let mut task = String::new();

        io::stdin()
            .read_line(&mut task)
            .expect("Could not capture Task number");

        let task = match task.trim().parse::<usize>() {
            Ok(num) => { 
                if num > 0 && num <= todos.len() {
                    num
                } else { return Err(TodoError::TodoNotFound)}
            },
            Err(e) => return Err(TodoError::InvalidInput(e)),
        };

        if let Some(todo) = todos.get_mut(task - 1) {
            todo.status = Status::Finished;
            println!("\"{}\" marked as \'Finished\'\n", todo.task);
        };

        Ok(())

    }

    pub fn delete(&mut self) -> Result<(), TodoError> {
     
        self.print_list()?;

        let todos = &mut self.todos;

        println!("\nEnter the task number to be Deleted");
        let mut task = String::new();

        io::stdin()
            .read_line(&mut task)
            .expect("Could not capture Task number");

        let todo_num = match task.trim().parse::<usize>() {
            Ok(num) => {
                if num > 0 && num <= todos.len() {
                    num 
                } else { return Err(TodoError::TodoNotFound); }
            },
            Err(e) => return Err(TodoError::InvalidInput(e)),
        };

        let todo = todos.remove(todo_num - 1);
        println!("Todo ==> \"{}\" has been \'Deleted\'\n", todo.task.trim());

        Ok(())
    }

    pub fn print_list(&self) -> Result<(), TodoError>{

        if !self.todos.is_empty() {
            println!(" Todo List ==>");
            for (count,todo) in self.todos.iter().enumerate(){
                println!("|{} ===> {} => {:?}", count+1, todo.task.trim(), todo.status);
            }
            println!();
        }
        else { 
            return Err(TodoError::EmptyList); 
        }

        Ok(())
    }
}
