use std::fmt::{Debug, Display, Formatter};
use std::io::Error;
use std::net::{SocketAddr};
use std::result;
use bytes::{Buf, BytesMut};
use tokio;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, oneshot};
use tokio::sync::mpsc::error::SendError;
use crate::message;
use crate::message::Message;


// ------ 错误封装 ------ //
#[derive(Debug)]
pub enum VPadServerError {
    IOError(Error),
    CloseError(SendError<()>)
}
pub type Result = result::Result<(), VPadServerError>;
impl From<SendError<()>> for VPadServerError {
    fn from(value: SendError<()>) -> Self {
        VPadServerError::CloseError(value)
    }
}
impl From<Error> for VPadServerError {
    fn from(value: Error) -> Self {
        VPadServerError::IOError(value)
    }
}
// ------ 错误封装 ------ //

// type OneShotChan = (oneshot::Sender<()>, oneshot::Receiver<()>);
type MPSCChan = (mpsc::Sender<()>, mpsc::Receiver<()>);

pub struct VPadServer<'a> {
    pub addr: &'a str,
    close_channel: MPSCChan,
}

impl<'a> VPadServer<'a> {
    pub fn bind(addr: &'a str)  -> VPadServer {
        VPadServer {
            addr,
            close_channel: mpsc::channel(1)
        }
    }

    pub async fn start(self) -> Result {
        let (_tx, mut rx) = self.close_channel;
        let listener = TcpListener::bind(self.addr).await?;
        loop {
            tokio::select! {
                Ok((mut socket, addr)) = listener.accept() => {
                    tokio::spawn(process_socket(socket, addr));
                },
                _ = rx.recv() => {
                    break;
                }
            }
        }
        Ok(())
    }

    pub async fn close(self) -> Result {
        self.close_channel.0.send(()).await?;
        Ok(())
    }

}

async fn process_socket(mut socket: TcpStream, addr: SocketAddr) {
    println!("{:?}", addr);
    let (mut rd, mut wr) = socket.split();
    let mut byte_buf = BytesMut::with_capacity(1024);
    while let Ok(len) = rd.read_buf(&mut byte_buf).await {
        // Eof return
        if len == 0 {
            break;
        }
        // 具有并且具有完整的一条消息（待修改）
        while byte_buf.has_remaining() {
            if let Some(msg) = Message::parse(&mut byte_buf) {
                println!("{:?}", msg);
                if let Some(return_msg) = msg.handle_and_return() {
                    println!("got return msg => {:?}", return_msg);
                    wr.write_buf(&mut return_msg.to_buf()).await;
                    wr.flush().await;
                }
            }
        }
        byte_buf = BytesMut::with_capacity(1024);
    }
}