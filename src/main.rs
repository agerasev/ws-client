extern crate net2;
extern crate websocket_reuseaddr as websocket;

use std::net::{SocketAddrV4, Ipv4Addr};

use net2::{TcpBuilder};

use websocket::{Client, Message, /*Sender, */Receiver};
use websocket::client::request::Url;

fn main() {
	let url = Url::parse("ws://tracker-nthend.rhcloud.com:8000/peers").unwrap();
	let request = Client::connect(url).unwrap();
	let response = request.send().unwrap();
	response.validate().unwrap();

	let addr = response.get_writer().local_addr().unwrap();
	println!("{}", addr);


	let tcp = TcpBuilder::new_v4().unwrap();
	tcp.reuse_address(true).unwrap().only_v6(false).unwrap();

	let listener = tcp.bind(SocketAddrV4::new(Ipv4Addr::new(0,0,0,0), addr.port())).unwrap().listen(128).unwrap();

	for stream in listener.incoming() {
		match stream {
			Ok(stream) => { println!("incoming {}", stream.peer_addr().unwrap()); }
			Err(_) => { println!("error listen"); }
		}
	}


	let /*mut*/ client = response.begin();
	let (_/*mut sender*/, mut receiver) = client.split();

	for msgres in receiver.incoming_messages() {
		let message: Message = msgres.unwrap();
		println!("{:?}", std::str::from_utf8(&message.payload).unwrap());
		// sender.send_message(&message).unwrap();
	}
}