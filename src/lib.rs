#![forbid(unsafe_code)]

use std::time::Duration;
// Main Library file
use sabaton_mw::{NodeBuilder, error::MiddlewareError};
use tracing::{debug, info, span, Level,error, info_span, error_span};

pub struct Params<'a> {
    pub maybe_group: Option<&'a str>,
    pub maybe_instance: Option<&'a str>,
    // add additional parameters here
}

pub fn example_node_main(params: &Params) -> Result<(),MiddlewareError> {

    let mut node_builder =   NodeBuilder::default();

        //.multi_threaded()  Enable this if you want a multi-threaded runtime
        //.with_num_workers(4)    // Number of work threads. Fixed to 1 for single threaded runtime.

    if let Some(group) = params.maybe_group {
        node_builder = node_builder.with_group(group.into());
    }

    if let Some(instance) = params.maybe_instance {
        node_builder = node_builder.with_instance(instance.into());
    }

    let node = node_builder.build("example-node".to_owned()).expect("Node creation error");

    let res = node.spin(|| {
        
        let span = span!(target: "MAIN", Level::DEBUG, "Application Main Loop");
        let _root_span = span.enter();
        info!("Application Main Loop Started with tick interval 100mS");

        let mut ticker = tokio::time::interval(Duration::from_millis(100));

        let _task = tokio::spawn( async move {

            let inner_span = info_span!("inner");
            let _inner_entered = inner_span.enter();
        
            info!(a_bool = true, answer = 42, message = "first example");

            loop {
                let _ = ticker.tick().await;
                debug!("Tick");
            }

         });
         
    });


    res

}
