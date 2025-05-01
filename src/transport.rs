use crate::cli::TransportType;
use servicepoint::{FakeConnection, Packet, UdpSocketExt};
use std::fmt::Debug;
use std::net::{TcpStream, UdpSocket};
use std::sync::Mutex;
use tungstenite::client::IntoClientRequest;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{ClientRequestBuilder, WebSocket};

#[derive(Debug)]
pub enum Transport {
    Fake,
    Udp(UdpSocket),
    WebSocket(Mutex<WebSocket<MaybeTlsStream<TcpStream>>>),
}

impl Transport {
    pub fn connect(kind: TransportType, destination: &str) -> Transport {
        match kind {
            TransportType::Udp => {
                Self::Udp(UdpSocket::bind_connect(destination).expect("failed to bind socket"))
            }
            TransportType::WebSocket => {
                let request = ClientRequestBuilder::new(
                    destination.parse().expect("Invalid destination url"),
                )
                .into_client_request()
                .unwrap();
                let (sock, _) =
                    tungstenite::connect(request).expect("failed to connect to websocket");
                Self::WebSocket(Mutex::new(sock))
            }
            TransportType::Fake => Self::Fake,
        }
    }

    pub(crate) fn send_command<T: TryInto<Packet>>(&self, command: T) -> Option<()>
    where
        <T as TryInto<Packet>>::Error: Debug,
    {
        match self {
            Self::Udp(socket) => socket.send_command(command),
            Self::WebSocket(socket) => {
                let bytes: Vec<u8> = command.try_into().unwrap().into();
                let mut socket = socket.lock().unwrap();
                socket.send(tungstenite::Message::Binary(bytes.into())).ok()
            }
            Self::Fake => FakeConnection.send_command(command),
        }
    }
}
