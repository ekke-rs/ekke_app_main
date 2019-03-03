use actix             :: { prelude::*                                        };
use futures_util      :: { future::FutureExt, try_future::TryFutureExt       };
use slog              :: { Logger, info, o                                   };
use tokio_async_await :: { await                                             };
use typename          :: { TypeName                                          };
use libekke::services :: { RegisterApplication, RegisterApplicationResponse  };

use ekke_io::
{
	  IpcMessage
	, Rpc
	, ResultExtSlog
	, RegisterServiceMethod
	, ConnID
	, SendRequest
	, MessageType
};

use libekke::Ekke;


mod     http_request   ;
pub use http_request::*;


#[ derive( Debug, Clone, TypeName ) ]
//
pub struct MainUi
{
	pub log: Logger
}

impl Actor for MainUi
{
	type Context = Context<Self>;

	// Start the server
	// Register our services with the dispatcher
	//
	fn started( &mut self, ctx: &mut Self::Context )
	{
		// let _our_address = ctx.address().clone();
		let log  = self.log.clone();

		let rpc  = Rpc::new( log.new( o!( "Actor" => "Rpc" ) ), crate::service_map ).start();
		let rpc2 = rpc.clone();

		self.register_service::<HttpRequest>( &rpc, ctx );

		let program = async move
		{
			info!( log, "Ekke MainUi Starting up" );

			// Create an IpcPeer representing the Ekke Server
			//
			let ekke_server = await!( Ekke::server_peer( log.new( o!( "IpcPeer" => "EkkeServer" ) ), rpc ) );


			// Register our application with the server
			//
			let conn_id = ConnID::new();

			let msg = RegisterApplication
			{
				conn_id                                       ,
				app_name: "MainUi".to_string()                ,
				route   : "/mainui".to_string()               ,
				services: vec![ "HttpRequest".to_string() ] ,
			};


			let response = await!( rpc2.send
			(
				SendRequest
				{
					ipc_peer: ekke_server.recipient(),

					ipc_msg: IpcMessage::new
					(
						  "RegisterApplication".to_string()
						, msg
						, MessageType::SendRequest
						, conn_id
					)
				}

			)).unwraps( &log );

			let resp: RegisterApplicationResponse = Rpc::deserialize( response.ipc_msg.payload ).unwraps( &log );

			info!( log, "MainUi: Received response for RegisterApplication: {}", resp.response );

			Ok(())
		};

		Arbiter::spawn( program.boxed().compat() );
	}
}