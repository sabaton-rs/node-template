#![forbid(unsafe_code)]

use std::time::Duration;
// Main Library file
use sabaton_mw::{NodeBuilder, error::MiddlewareError};
use tracing::{debug, info, span, Level};

pub fn example_node_main() -> Result<(),MiddlewareError> {

    let node =   NodeBuilder::default()
        //.multi_threaded()  Enable this if you want a multi-threaded runtime
        //.with_num_workers(4)    // Number of work threads. Fixed to 1 for single threaded runtime.
        .build("example-node".to_owned()).expect("Node creation error");

    let res = node.spin(|| {
        
        span!(target: "MAIN", Level::TRACE, "Application Main Loop");
        info!("Application Main Loop Started with tick interval 100mS");

        let mut ticker = tokio::time::interval(Duration::from_millis(100));

        let _task = tokio::spawn( async move {

            loop {
                let _ = ticker.tick().await;
                debug!("Tick");
            }

         });
         
    });


    res

}
