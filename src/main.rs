use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

fn give_me_default<T>() -> T
where
    T: Default,
{
    Default::default()
}

static BIND_ADDR: &str = "localhost:6565";

#[tokio::main]
async fn main() {
    let value = give_me_default::<i32>();

    let listener = TcpListener::bind(BIND_ADDR).await.unwrap();

    let (tx, _rx) = broadcast::channel(10);

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (read, mut write) = socket.split();

            let mut reader = BufReader::new(read);
            let mut line = String::new();

            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            break;
                        }
                        tx.send((line.clone(), addr)).unwrap();
                        line.clear();
                    }
                    result = rx.recv() => {
                        let (msg, other_addr) = result.unwrap();
                        if other_addr != addr {
                        write.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }

                }
            }
        });
    }
}
