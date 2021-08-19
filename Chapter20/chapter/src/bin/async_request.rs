use async_std::io::prelude::*;
use async_std::{net, task};
use std::future::Future;
use std::io;

//async fn cheapo_request(host: &str, port: u16, path: &str) -> std::io::Result<String> {
//    let mut socket = net::TcpStream::connect((host, port)).await?;
//    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\n", path, host);
//
//    socket.write_all(request.as_bytes()).await?;
//    socket.shutdown(net::Shutdown::Write)?;
//
//    let mut response = String::new();
//    socket.read_to_string(&mut response).await?;
//
//    Ok(response)
//}

//fn cheapo_request<'a>(
//    host: &'a str,
//    port: u16,
//    path: &'a str,
//) -> impl Future<Output = io::Result<String>> + 'a {
//    async move {
//        let mut socket = net::TcpStream::connect((host, port)).await?;
//        let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\n", path, host);
//
//        socket.write_all(request.as_bytes()).await?;
//        socket.shutdown(net::Shutdown::Write)?;
//
//        let mut response = String::new();
//        socket.read_to_string(&mut response).await?;
//
//        Ok(response)
//    }
//}

fn cheapo_request(host: &str, port: u16, path: &str) -> impl Future<Output = io::Result<String>> {
    let host = host.to_string();
    let path = path.to_string();

    async move {
        let mut socket = net::TcpStream::connect((&*host, port)).await?;
        let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\n", path, host);

        socket.write_all(request.as_bytes()).await?;
        socket.shutdown(net::Shutdown::Write)?;

        let mut response = String::new();
        socket.read_to_string(&mut response).await?;

        Ok(response)
    }
}

//async fn owning_cheapo_request(host: String, port: u16, path: String) -> std::io::Result<String> {
//    cheapo_request(&host, port, &path).await
//}

async fn many_request(requests: Vec<(String, u16, String)>) -> Vec<std::io::Result<String>> {
    let mut handles = Vec::new();
    for (host, port, path) in requests {
        handles.push(task::spawn(async move {
            cheapo_request(&host, port, &path).await
        }));
    }

    let mut results = Vec::new();
    for handle in handles {
        results.push(handle.await);
    }

    results
}

fn main() -> std::io::Result<()> {
    //let response = task::block_on(cheapo_request("www.example.com", 80, "/"))?;
    //println!("response = {}", response);

    let requests = vec![
        ("example.com".to_string(), 80, "/".to_string()),
        ("google.com".to_string(), 80, "/".to_string()),
        ("4chan.org".to_string(), 80, "/".to_string()),
        ("reddit.com".to_string(), 80, "/".to_string()),
        ("wikipedia.net".to_string(), 80, "/".to_string()),
        ("holamundo".to_string(), 80, "/".to_string()),
    ];

    let results = task::block_on(many_request(requests));
    for result in results {
        match result {
            Ok(response) => println!("{}", response),
            Err(e) => eprintln!("Error: {}", e),
        };
        println!("========================");
    }

    Ok(())
}
