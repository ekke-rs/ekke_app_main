use actix             :: { prelude::*                                        };
use futures_util      :: { future::FutureExt, try_future::TryFutureExt       };
use slog              :: { Logger, info, o, error                            };
use slog_unwraps      :: { ResultExt                                         };
use tokio_async_await :: { await                                             };
use typename          :: { TypeName                                          };
use libekke::services :: { RegisterApplication, RegisterApplicationResponse  };
use libekke           :: { FrontendRequest                                   };

use ekke_io::
{
	  IpcMessage
	, Rpc
	, RegisterServiceMethod
	, ConnID
	, IpcRequestOut
	, MessageType
};

use libekke::Ekke;


mod     frontend_request   ;
pub use frontend_request::*;


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

		self.register_service::<FrontendRequest>( &rpc, ctx );

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
				app_name: "mainui".into()                     ,
				routes   : vec![ "/mainui".to_string()    ]           ,
				services: vec![ "FrontendRequest".to_string() ] ,
			};


			let response = await!( rpc2.send
			(
				IpcRequestOut
				{
					ipc_peer: ekke_server.recipient(),

					ipc_msg: IpcMessage::new
					(
						  "RegisterApplication".into()
						, msg
						, MessageType::IpcRequestOut
						, conn_id
					)
				}

			)).unwraps( &log ); // Actix::MailboxError


			match response
			{
				Ok ( r ) =>
				{
					let resp: RegisterApplicationResponse = Rpc::deserialize( r.ipc_msg.payload ).unwraps( &log );

					info!( log, "MainUi: Received response for RegisterApplication: {}", resp.response );
				},

				Err( e ) =>
				{
					error!( log, "MainUi: RegisterApplication failed: {}", e );
				}
			}



			Ok(())
		};

		Arbiter::spawn( program.boxed().compat() );
	}
}
