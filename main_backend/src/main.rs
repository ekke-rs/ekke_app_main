#![ forbid(unsafe_code) ]
#![ feature( await_macro, async_await, futures_api, nll ) ]
#![ warn(unused_extern_crates) ]

use libmainui  :: { MainUi                       };
use ekke_io    :: { ThreadLocalDrain             };
use actix      :: { prelude::*                   };

use slog       :: { Logger, Drain, o             };
use slog_term  :: { TermDecorator, CompactFormat };
use slog_async :: { Async                        };

// use log_panics ;



fn main()
{
	let sys = System::new( "ekke_main" );

	let log = root_logger().new( o!( "thread_name" => "main", "Actor" => "MainUi" ) );

	// log_panics::init();

	let _ekke = MainUi{ log }.start();

	sys.run();
}



fn root_logger() -> Logger
{
	let decorator = TermDecorator ::new().stdout()  .build()        ;
	let compact   = CompactFormat ::new( decorator ).build().fuse() ;
	let drain     = Async         ::new( compact   ).build().fuse() ;

	Logger::root( ThreadLocalDrain{ drain }.fuse(), o!( "version" => "0.1" ) )
}




