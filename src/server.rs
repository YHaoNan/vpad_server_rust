use std::fmt::{Debug};
use std::io::{Error};
use std::net::{IpAddr, SocketAddr};
use std::result;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{StreamExt, SinkExt};
use tokio;
use tokio::net::{TcpSocket, TcpStream};
use tokio::sync::{mpsc};
use tokio::sync::mpsc::error::SendError;
use tokio_util::codec::Framed;
use crate::message::Message;
use crate::message_codec::MessageCodec;

pub type Result = result::Result<(), VPadServerError>;

type MPSCChan = (mpsc::Sender<()>, mpsc::Receiver<()>);

pub struct VPadServer {
    pub ipaddr: IpAddr,
    pub port: u16,
    close_channel: MPSCChan,
}

impl VPadServer {
    pub fn bind(ipaddr:IpAddr, port: u16)  -> VPadServer {
        VPadServer {
            ipaddr, port,
            close_channel: mpsc::channel(1)
        }
    }

    pub async fn start(self) -> Result {
        let (_tx, mut rx) = self.close_channel;

        let socket = TcpSocket::new_v4()?;
        socket.set_reuseaddr(true)?;
        socket.bind(SocketAddr::new(self.ipaddr, self.port))?;

        let listener = socket.listen(1024)?;
        loop {
            // 在两个异步任务上轮询，第一个完成的任务的代码块将被执行，另一个代码块将被放弃
            tokio::select! {
                Ok((socket, addr)) = listener.accept() => {
                    tokio::spawn(process_socket(socket, addr));
                },
                _ = rx.recv() => {
                    break;
                }
            }
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn close(self) -> Result {
        self.close_channel.0.send(()).await?;
        Ok(())
    }

}

pub struct VPadMessageContext {
    pub addr: SocketAddr
}

type MessageFramedStream = SplitStream<Framed<TcpStream, MessageCodec>>;
type MessageFramedSink = SplitSink<Framed<TcpStream, MessageCodec>, Message>;

#[allow(unused_must_use)]
async fn process_socket(socket: TcpStream, addr: SocketAddr) {
    log::info!("Got a new connection from: {:?}", addr);

    let framed = Framed::new(socket, MessageCodec{});
    let (frame_writer, frame_reader) =
        framed.split::<Message>();

    let (msg_tx, msg_rx) = mpsc::channel::<Message>(4);

    let ctx = VPadMessageContext { addr };

    let mut read_task = tokio::spawn(async move {
        read_from_client(frame_reader, msg_tx, ctx).await;
    });

    let mut write_task = tokio::spawn(async move {
        write_to_client(frame_writer, msg_rx).await;
    });

    if tokio::try_join!(&mut read_task, &mut write_task).is_err() {
        log::info!("read/write task is terminated!");
        read_task.abort();
        write_task.abort();
    }
}

async fn read_from_client(mut reader: MessageFramedStream, msg_tx: mpsc::Sender<Message>, ctx: VPadMessageContext) {
    loop {
        match reader.next().await {
            None => {
                log::info!("Client closed");
                break;
            }
            Some(Err(e)) => {
                log::info!("Read from client error: {:?}", e);
            }
            Some(Ok(msg)) => {
                log::debug!("Got an message => {:?}", msg);
                if let Some(return_msg) = msg.handle_and_return(&ctx) {
                    log::debug!("Return msg => {:?}", return_msg);
                    if msg_tx.send(return_msg).await.is_err() {
                        log::error!("Error to send return msg to sender channel");
                    }
                }
            }
        }
    }
}

async fn write_to_client(mut writer: MessageFramedSink, mut msg_rx: mpsc::Receiver<Message>) {
    while let Some(msg) = msg_rx.recv().await {
        if writer.send(msg).await.is_err() {
            log::error!("Error to sink msg to client");
        }
    }
}

// ------ 错误封装 ------ //
#[derive(Debug)]
pub enum VPadServerError {
    IOError(Error),
    CloseError(SendError<()>)
}
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
