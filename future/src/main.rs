use tokio::{task, time::{sleep, Duration}};

#[tokio::main]
async fn main() {
    /*println!("Calling sleep for the first time");
    let fut1 = sleep(Duration::from_secs(2));

    println!("Calling sleep for the second time");
    let fut2 = sleep(Duration::from_secs(2));

    tokio::join!(fut1, fut2);

    println!("Both futures finished executing");*/

    let task1 = task::spawn(async {
        println!("Task 1 started.");
        sleep(Duration::from_secs(2)).await;
        println!("Task 1 is finished.")
    });

    let task2 = task::spawn(async {
        println!("Task 2 started.");
        sleep(Duration::from_secs(2)).await;
        println!("Task 2 is finished.")
    });

    let task3 = task::spawn(async {
        println!("Task 3 started.");
        sleep(Duration::from_secs(2)).await;
        println!("Task 3 is finished.")
    });

    task1.await.unwrap();
    task2.await.unwrap();
    task3.await.unwrap();

    println!("All tasks are finished!");
}

