use std::any::Any;
use std::sync::Arc;
use std::time::Duration;
use zeroconf::prelude::*;
use zeroconf::{MdnsBrowser, ServiceDiscovery};

pub fn browse() {
    let mut browser = MdnsBrowser::new("_http._tcp");

    browser.set_service_discovered_callback(Box::new(on_service_discovered));

    let event_loop = browser.browse_services().unwrap();

    loop {
        // calling `poll()` will keep this browser alive
        event_loop.poll(Duration::from_secs(0)).unwrap();
    }
}

fn on_service_discovered(
    result: zeroconf::Result<ServiceDiscovery>,
    _context: Option<Arc<dyn Any>>,
) {
    println!("Service discovered: {:?}", result.unwrap());

    // ...
}

// Connect to a socket provided by zeroconf browse
use std::io::prelude::*;
use std::net::TcpStream;

fn drop_send() -> std::io::Result<()> {
    let address = format!("{}:{}", "127.0.0.1", "12345");
    let mut stream = TcpStream::connect(address)?;

    stream.write(&[1])?;
    stream.read(&mut [0; 128])?;

    Ok(())
}

pub fn drop() {
    let result = drop_send();
    match result {
        Err(e) => { // NOTE(Able): Error handling, Rarely do I do that
            println!("Error: {:?}", e.kind());
        }
        Ok(()) => {
            println!("File Dropped!");
        }
    }
}

