// fn main() {
//     // env_logger::init();
//     for dev in nusb::list_devices().unwrap() {
//         println!("{:#?}", dev);
//     }

    
//     // if vid != 0x2341 {
//         // continue;
//     // }
// }


use futures_lite::future::block_on;
use nusb::transfer::RequestBuffer;

fn main() {
    // env_logger::init();
    let di = nusb::list_devices()
        .unwrap()
        // .find(|d| d.vendor_id() == 0x2341 && d.product_id() == 0x0a23)
        .find(|d| d.vendor_id() == 0x2341)
        .expect("device should be connected");

    println!("Device info: {di:?}");

    let device = di.open().unwrap();
    let interface = device.claim_interface(0).unwrap();

    block_on(interface.bulk_out(0x02, Vec::from([1, 2, 3, 4, 5])))
        .into_result()
        .unwrap();

    let mut queue = interface.bulk_in_queue(0x81);

    loop {
        while queue.pending() < 8 {
            queue.submit(RequestBuffer::new(256));
        }
        let result = block_on(queue.next_complete());
        println!("{result:?}");
        if result.status.is_err() {
            break;
        }
    }
}

