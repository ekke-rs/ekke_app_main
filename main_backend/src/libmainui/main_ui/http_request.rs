use typename          :: { TypeName                        };
use actix             :: { prelude::*                      };
use serde_derive      :: { Serialize, Deserialize          };
use ekke_io           :: { IpcMessage, ConnID, MessageType };
use slog              :: { info                            };

use crate::MainUi;

#[ derive( Debug, Clone, Serialize, Deserialize, Message, TypeName ) ] #[rtype(result="IpcMessage")]
//
pub struct HttpRequest
{
	pub conn_id : ConnID,
	pub app_name: String,
}

#[ derive( Debug, Clone, Serialize, Deserialize, Message, TypeName ) ]
//
pub struct HttpResponse
{
	pub body: String
}



impl Handler<HttpRequest> for MainUi
{
	type Result = IpcMessage;

	fn handle( &mut self, msg: HttpRequest, _ctx: &mut Context<Self> ) -> Self::Result
	{
		info!( self.log, "MainUi: Received app registration for app: {}", msg.app_name );

		let service = "HttpResponse".to_string();
		let payload = HttpResponse{ body: "<html><body>MainUi</body></html>".to_string() };
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

