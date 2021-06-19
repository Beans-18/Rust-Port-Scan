use std::sync::{Arc, Mutex};
use tokio::net::TcpStream;
use futures;

#[tokio::main]
pub async fn main() {
    let access: Arc<Mutex<Vec<u32>>> = Arc::new(Mutex::new(Vec::new()));
    let inner_access = Arc::clone(&access);
    let handle = tokio::spawn(async move {
        let mut tasks = vec![];
        for i in 0..1000 {
            let access_clone = Arc::clone(&inner_access);
            let i: u32 = i;
            tasks.push(tokio::spawn(async move {
                try_connection(access_clone, "127.0.0.1".to_string(), i.to_string()).await;
            }));
        }
        tasks
    });

    let tasks = handle.await.unwrap();
    futures::future::join_all(tasks).await;

    println!("{:?}", access);
}

async fn try_connection(check: Arc<Mutex<Vec<u32>>>, ip: String, port: String) {
    match TcpStream::connect(format!("{}:{}", ip, port)).await {
        Ok(_) => {
            let mut vec = check.lock().unwrap();
            vec.push(port.parse::<u32>().unwrap());
        }
        Err(e) => println!("{}", e),
    }
}
