use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;


pub struct Server {
    addr: String,
}

impl Server {

    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub async fn run(&self) {
        let listener = TcpListener::bind(&self.addr).await.unwrap();
        println!("Listening server on: {}", &self.addr);
        loop {
            let (socket, _) = listener.accept().await.unwrap();
            self.process(socket).await;
        }
    }

    async fn process(&self, mut socket: TcpStream) {
        let mut buff: [u8;1024] = [0; 1024];
        match socket.read(&mut buff).await {
            Ok(_) => {
                println!("{}", String::from_utf8_lossy(&buff));
                let response = "HTTP/1.1 200 OK\r\n\r\n<h1>Hello WOrld<h2>";
                match socket.write(response.as_bytes()).await {
                    Ok(_) => println!("Response sent ok"),
                    Err(e) => println!("Error occured during response: {}", e)
                }
            },
            Err(e) => println!("Error occured during reading: {}", e)
        }
        
    }
    
}