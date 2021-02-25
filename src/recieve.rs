use std::any::Any;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use zeroconf::prelude::*;
use zeroconf::{MdnsService, ServiceRegistration, TxtRecord};

#[derive(Default, Debug)]
pub struct Context {
    service_name: String,
}

pub fn register() {
    let mut service = MdnsService::new("_http._tcp", 8080);
    let context: Arc<Mutex<Context>> = Arc::default();

    /*
        let mut txt_record = TxtRecord::new();
        txt_record.insert("foo", "bar").unwrap();
        service.set_txt_record(txt_record);
    */

    service.set_registered_callback(Box::new(on_service_registered));
    service.set_context(Box::new(context));
    let event_loop = service.register().unwrap();

    loop {
        // calling `poll()` will keep this service alive
        event_loop.poll(Duration::from_secs(0)).unwrap();
    }
}

fn on_service_registered(
    result: zeroconf::Result<ServiceRegistration>,
    context: Option<Arc<dyn Any>>,
) {
    let service = result.unwrap();

    println!("Service registered: {:?}", service);

    let context = context
        .as_ref()
        .unwrap()
        .downcast_ref::<Arc<Mutex<Context>>>()
        .unwrap()
        .clone();

    context.lock().unwrap().service_name = service.name().clone();

    println!("Context: {:?}", context);

    // ...
}
