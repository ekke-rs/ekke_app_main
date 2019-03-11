use actix             :: { prelude::*                               };
use ekke_io           :: { IpcMessage, MessageType                  };
use slog              :: { info                                     };
use libekke           :: { FrontendRequest, BackendResponse, ResponseStatus };

use crate::MainUi;




impl Handler<FrontendRequest> for MainUi
{
	type Result = IpcMessage;

	fn handle( &mut self, msg: FrontendRequest, _ctx: &mut Context<Self> ) -> Self::Result
	{
		info!( self.log, "MainUi: Received frontend request for path: {}", msg.path );

		let service = "BackendResponse".to_string();
		let payload = BackendResponse{ status: ResponseStatus::Ok, body: "<html><body>MainUi</body></html>".as_bytes().to_vec() };
		let ms_type = MessageType::Response;
		let conn_id = msg.conn_id;

		IpcMessage::new
		(
			  service
			, payload
			, ms_type
			, conn_id
		)
	}
}

