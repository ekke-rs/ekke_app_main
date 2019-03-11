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
	pub use crate::main_ui::HttpRequest;
}




use
{
	crate   :: { services::*      } ,
	ekke_io :: { IpcMessage, Rpc  } ,
	actix   :: { Recipient        } ,
	slog    :: { Logger, error    } ,
};


pub(crate) fn service_map( rpc: &Rpc, log: Logger, msg: IpcMessage, ipc_peer: Recipient< IpcMessage > )
{
    match msg.service.as_ref()
    {
      	"HttpRequest" => rpc.deser_into::<HttpRequest>( msg, ipc_peer ),
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
