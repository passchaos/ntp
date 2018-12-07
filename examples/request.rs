//! How to request an NTP packet from an NTP server.

extern crate chrono;
extern crate ntp;
extern crate tokio_core;
extern crate futures;

use std::net::ToSocketAddrs;
use chrono::TimeZone;
use futures::future::Future;

fn local_time(timestamp: ntp::protocol::TimestampFormat) -> chrono::DateTime<chrono::Local> {
    let unix_time = ntp::unix_time::Instant::from(timestamp);
    chrono::Local.timestamp(unix_time.secs(), unix_time.subsec_nanos() as _)
}

fn main() {
    let address = "0.pool.ntp.org:123";
    let response: ntp::protocol::Packet = ntp::request(address).unwrap();
    println!("Timestamps in local time:");
    println!("  reference: {}", local_time(response.reference_timestamp));
    println!("  origin:    {}", local_time(response.origin_timestamp));
    println!("  receive:   {}", local_time(response.receive_timestamp));
    println!("  transmit:  {}", local_time(response.transmit_timestamp));

    let mut core = tokio_core::reactor::Core::new().unwrap();

    let mut futs = vec![];
    for addr in vec![   "cn.ntp.org.cn:123",
                         "time-a-g.nist.gov:123",
                         "ntp.byted.org:123",
                         "time.windows.com:123",
                         "ntp.aliyun.com:123"] {
        let address = "time.windows.com:123".to_socket_addrs().unwrap().next().unwrap();

        let f = ntp::request_async(address, &core.handle()).unwrap()
            .and_then(|item| {
                println!("item: {:?}", item);
                Ok(())
            });
        futs.push(f);
    }
    let f = futures::future::join_all(futs);

    core.run(f);
}
