use tokio;
use std::{collections::HashMap, io};
use uuid::Uuid;
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone, Copy)]
enum Taskstate {
    Waiting,
    Running,
    Cancelled,
    Finished,
}

#[derive(Debug, Clone)]
struct Task {
    name: String,
    state:Taskstate,
    cancel_token: CancellationToken,
}


#[tokio::main]
async fn main() {
    let mut task: HashMap<Uuid,Task> = HashMap::new();
    loop{
        let mut k = String::new();
        println!("Enter the command you want to do ( add / list / execute /  cancel / exit ):");
        io::stdin().read_line(&mut k).expect("Expected a command");
        k = k.to_lowercase();
        match k.as_str().trim() {
            "add" => {
                add_task(&mut task).await;
            }
            "list" =>{
                list_task(&mut task);
            }
            "execute" =>{
                let mut id_in = String::new();
                println!("Enter the Id to execute");
                io::stdin().read_line(&mut id_in).expect("Expected a valid ID");
                let id = Uuid::parse_str(id_in.trim()).expect("Not a valid ID");
                exe_task(id, &mut task);
            }
            "cancel" =>{
                let mut id_in = String::new();
                println!("Enter the Id to cancel");
                io::stdin().read_line(&mut id_in).expect("Expected a valid ID");
                let id = Uuid::parse_str(id_in.trim()).expect("Not a valid ID");
                cancel_task(id, &mut task);
            }
            "exit"=> {
                break;
            }
            _ => {
                println!("Unknown command");
            }

        }
    }
}

async fn add_task( task: &mut HashMap<Uuid, Task>) {
    let mut x = String::new();
    println!("Enter the number of task to be started at first");
    io::stdin().read_line(&mut x).expect("Expected a number");
    let num:i32 = x.trim().parse().expect("Number not numbering");
    for i in 0..num {
        let mut s = String::new();
        println!("Enter the Name of the task");
        io::stdin().read_line(&mut s).expect("Expected a String");
        let id = Uuid::new_v4();
        task.insert(id, Task {
            name: s,
            state: Taskstate::Waiting,
            cancel_token : CancellationToken::new(),
        });
        println!("Task added with ID {}", id);

    }
}

fn list_task(task: &mut HashMap<Uuid, Task>){
    println!("The Registered Tasks are as follows: \n {:?}", task);
}

fn exe_task(id: Uuid, task: &mut HashMap<Uuid, Task>){
    if let Some(task) = task.get_mut(&id){
        let name = task.name.clone();
        let token = task.cancel_token.clone();
        match task.state {
            Taskstate::Waiting | Taskstate::Running => {
                task.state = Taskstate::Running;
                tokio::spawn(async move { 
                    loop {
                    tokio::select! {
                        _ = token.cancelled() => {
                            println!("Task {} cancelled", name.trim());
                            break;
                        }
                        _ = tokio::time::sleep(std::time::Duration::from_secs(5)) => {
                            println!("Task {} is running..", name.trim());
                        }
                    }}

                }
                );
            }
            Taskstate::Cancelled | Taskstate::Finished => { println!("Task is already cancelled or finished so therefore cant be executed !");
        }
        
    }
}else{
    println!("Task ID not found !");
}
}

fn cancel_task(id: Uuid, task: &mut HashMap<Uuid, Task>){
    if let Some(task) = task.get_mut(&id){
        task.state = Taskstate::Cancelled;
    }
    if let Some(task) = task.remove(&id){
        task.cancel_token.cancel();
        println!("Task {} cancelled and cant be retrieved !", task.name);
    }else {
        println!("Task ID not found !")
    }
}
