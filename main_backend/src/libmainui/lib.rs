//! This is the actual functionality for the ekke framework server. The binary contains just a very basic main function. All functionality is exposed through this library so you could build against it if needed.
//
#![ feature( await_macro, async_await, futures_api, nll, stmt_expr_attributes, never_type ) ]

mod main_ui;

pub use main_ui::
{
	MainUi
};



pub mod services
{
	pub use ekke_core::FrontendRequest;
}




use
{
	crate  :: { services::*, import::*  } ,
};


pub(crate) fn service_map( rpc: &Rpc, log: Logger, msg: IpcMessage, ipc_peer: Recipient< IpcMessage > )
{
	match msg.service.as_ref()
	{
		"FrontendRequest" => rpc.deser_into::<FrontendRequest>( msg, ipc_peer ),
		_             =>
		{
			let error = format!( "MainUi: Received request for unknown service: {}", &msg.service );

			error!( &log, "{}", &error );

			rpc.error_response
			(
				msg.service ,
				error       ,
				ipc_peer    ,
				msg.conn_id ,
			);
		}
	}
}


mod import
{
	#[ allow( unused_imports ) ]
	//
	pub( crate ) use
	{
		ekke_io           :: { ConnID, IpcMessage, IpcRequestOut, MessageType, Rpc },
		ekke_core         :: { BackendResponse, Ekke, ResponseStatus },
		actix             :: { Actor, Message, Handler, Context, Arbiter, Recipient, Addr                                        },
		futures_util      :: { future::FutureExt, try_future::TryFutureExt       },
		slog              :: { Logger, info, o, error                            },
		slog_unwraps      :: { ResultExt                                         },
		tokio_async_await :: { await as awaits                                   },
		typename          :: { TypeName                                          },
		ekke_core::services :: { RegisterApplication, RegisterApplicationResponse  },
	};
}
