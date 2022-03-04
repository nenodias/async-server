use tokio::net::{TcpListener, TcpStream};
use crate::http::{Request, Response, StatusCode};
use tokio::io::{AsyncReadExt};


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
        let mut buff: [u8;2048] = [0; 2048];
        match socket.read(&mut buff).await {
            Ok(_) => {
                if let Ok(req) = Request::try_from(&buff[..]) {
                    println!("{}", req.path());
                    let response = Response::new(StatusCode::Ok, Option::Some("Hello World!!!".to_string()));

                    match response.send( &mut socket).await {
                        Ok(_) => println!("Response sent ok"),
                        Err(e) => println!("Error occured during response: {}", e)
                    }
                }
            },
            Err(e) => println!("Error occured during reading: {}", e)
        }
        
    }
    
}