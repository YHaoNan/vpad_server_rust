use std::fmt::{Debug};
use std::io::{Cursor, Error, Read};
use std::net::{SocketAddr};
use std::result;
use std::sync::{Arc, Mutex};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use tokio;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc};
use tokio::sync::mpsc::error::SendError;
use crate::message::Message;
use crate::midi_connect::{GLOBAL_MIDI_CONNECTOR, MidiConnector};

pub type Result = result::Result<(), VPadServerError>;

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

pub struct VPadMessageContext<'a> {
    pub addr: &'a SocketAddr
}

#[allow(unused_must_use)]
async fn process_socket(mut socket: TcpStream, addr: SocketAddr) {
    println!("{:?}", addr);
    let (mut rd, mut wr) = socket.split();
    let mut byte_buf = BytesMut::with_capacity(1024);

    let ctx = VPadMessageContext {
        addr: &addr
    };

    while let Ok(len) = rd.read_buf(&mut byte_buf).await {
        // Eof return
        if len == 0 {
            continue;
        }
        println!("len: {}, buf: {:?}", len, byte_buf);
        // 具有并且具有完整的一条消息（待修改），目前不考虑消息不完整的情况
        while byte_buf.has_remaining() {
            // 解包content_bytes
            let content_bytes_cnt = byte_buf.get_i16();
            let mut this_message_chunk = byte_buf.chunks_exact(content_bytes_cnt as usize);
            if let Some(this_message_bytes) = this_message_chunk.next() {
                if let Some(msg) = Message::parse(BytesMut::from(this_message_bytes)) {
                    println!("{:?}", msg);
                    if let Some(return_msg) = msg.handle_and_return(&ctx) {
                        println!("got return msg => {:?}", return_msg);
                        // 这两个Future返回的Result不必须被处理，如它们是Err，忽略
                        wr.write_buf(&mut return_msg.to_buf()).await;
                        wr.flush().await;
                    }
                }
                byte_buf.advance(content_bytes_cnt as usize);
            }
        }
        byte_buf = BytesMut::with_capacity(1024);
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
