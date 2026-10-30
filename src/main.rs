use std::env;
#[derive(Debug)]
struct Task{
    id : usize,
    title : String
}



fn add_task(tasks : &mut Vec<Task>, title: String  ){
    let new_task = Task  {
        id : tasks.len()+1,
        title : title
    };

    tasks.push(new_task);

    
    println!("TASKS {:?}", tasks );

    println!("Tasks added Successfully !");

}

fn list_task(tasks : &Vec<Task>){
    if tasks.is_empty(){
        println!("No Task Added");
    }

    for task in tasks{
         println!("{}. {}", task.id, task.title);
    }
}

fn delete_task(tasks : &mut Vec<Task> , id : usize){

    let index = tasks.iter().position(|task| task.id == id);

    match index{
        Some(index) => {
            tasks.remove(index);
            println!("Task Deleted Successfully");
            for task in tasks{
         println!("{}. {}", task.id, task.title);
    }
        }
        None => {
            println!("Invalid ID");
        }
    }


}

fn main(){
    let mut tasks : Vec<Task> = Vec::new();
   

    let arguments : Vec<String> = env::args().collect();
    let command : &str = &arguments[1];



    match command{
        "add" => {
            println!("The operation to be performed is Addition");
            let title : String = arguments[2].clone();
            add_task(&mut tasks,title);
        }

        "list" => {
            println!("The operation to be performed is List");
            add_task(&mut tasks, String::from("Understand Ownership"));
            add_task(&mut tasks, String::from("Build CLI"));
            list_task(&tasks);
        }

        "delete" => {
            println!("The operation to be performed is Delete");
            let id: usize = arguments[2].parse().expect("REASON");
            add_task(&mut tasks, String::from("Understand Ownership"));
            add_task(&mut tasks, String::from("Build CLI"));
            delete_task(&mut tasks, id )
        }

        _ => {
            println!("This is a wild card entry ")
        }
    }
}
