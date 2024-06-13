// fn main() {
//     // env_logger::init();
//     for dev in nusb::list_devices().unwrap() {
//         println!("{:#?}", dev);
//     }

    
//     // if vid != 0x2341 {
//         // continue;
//     // }
// }

const NUMBER_OF_HEADER_BYTES: usize = 12;



use futures_lite::future::{block_on, BoxedLocal};
use byteorder::{ByteOrder, LittleEndian};
use nusb::transfer::RequestBuffer;


const USBTMC_MSGID_DEV_DEP_MSG_OUT: u8 = 1;
const USBTMC_MSGID_DEV_DEP_MSG_IN: u8 = 2;

// The Host must set bTag such that 1<=bTag<=255
static mut LASTBTAG: u8 = 0;

// 4 bytes header !
fn pack_bulk_out_header(msgid: u8) -> Vec<u8> {
    let btag: u8 = (unsafe { LASTBTAG } % 255) + 1;
    unsafe { LASTBTAG = btag };

    // BBBx
    vec![msgid, btag, !btag & 0xFF, 0x00]
}

fn pack_dev_dep_msg_out_header(transfer_size: usize, eom: bool) -> Vec<u8> {
    let mut hdr = pack_bulk_out_header(USBTMC_MSGID_DEV_DEP_MSG_OUT);

    hdr.append(&mut little_write_u32(transfer_size as u32, 4));
    hdr.push(if eom { 0x01 } else { 0x00 });
    hdr.append(&mut vec![0x00; 3]);

    hdr
}

fn pack_dev_dep_msg_in_header(transfer_size: usize, term_char: u8) -> Vec<u8> {
    let mut hdr = pack_bulk_out_header(USBTMC_MSGID_DEV_DEP_MSG_IN);

    hdr.append(&mut little_write_u32(transfer_size as u32, 4));
    hdr.push(if term_char == 0 { 0x00 } else { 0x02 });
    hdr.push(term_char);
    hdr.append(&mut vec![0x00; 2]);

    hdr
}

fn little_write_u32(size: u32, len: u8) -> Vec<u8> {
    let mut buf = vec![0; len as usize];
    LittleEndian::write_u32(&mut buf, size);

    buf
}

fn main() {

    let offset: usize = 0;
    let mut eom: bool = false;
    let max_transfer_size: u32 = 64;

    // env_logger::init();
    let di = nusb::list_devices()
        .unwrap()
        // .find(|d| d.vendor_id() == 0x2341 && d.product_id() == 0x0a23)
        .find(|d| d.vendor_id() == 0x1313)
        .expect("device should be connected");

    println!("Device info: {di:?}");

    let device = di.open().unwrap();
    let interface = device.claim_interface(0).unwrap();
    interface.clear_halt(0x02).unwrap();

    // let mut data: [u8; 64] = [0; 64]; // Initialize the array with 64 zeros
    // // Copy the string "*idn?" into the array
    // data[.."*IDN?".len()].copy_from_slice("*IDN?".as_bytes());

    let data = "*IDN?".as_bytes();

    let mut num: usize = data.len();

    while num > 0 {
        if num <= max_transfer_size as usize {
            eom = true;
        }

        let block = &data[offset..(num - offset)];
        let size: usize = block.len();
        println!("{}", size);

        let mut req = pack_dev_dep_msg_out_header(size, eom);
        let mut b: Vec<u8> = block.iter().cloned().collect();
        req.append(&mut b);
        // align block on 4 bytes
        req.append(&mut vec![0x00; (4 - (size % 4)) % 4]);

        let tott = block_on(interface.bulk_out(0x02, req.to_vec()))
            .into_result()
            .unwrap();
        println!("{:?}", tott);

        num = num - size;

        // let send = pack_dev_dep_msg_in_header(max_transfer_size as usize, 0);
        // tott.write_bulk(endpoint.address, &send, timeout)?;
    }


    let send = pack_dev_dep_msg_in_header(max_transfer_size as usize, 0);
    let tott2 = block_on(interface.bulk_out(0x02, send.to_vec()))
    .into_result()
    .unwrap();
    println!("- {:?}", tott2);


    let rb = nusb::transfer::RequestBuffer::new(max_transfer_size as usize);
    let queue = block_on(interface.bulk_in(0x82, rb)).into_result().unwrap();
    println!("{:?}", queue);

    let line_size = queue
        .iter()
        .skip(NUMBER_OF_HEADER_BYTES)
        .take_while(|c| **c != b'\n' && **c != b'\r')
        .count();

    let vvvvv = &queue[NUMBER_OF_HEADER_BYTES..line_size + NUMBER_OF_HEADER_BYTES];
    let result = String::from_utf8(vvvvv.to_vec()).unwrap().to_string();

    println!("{:?}", result);

    // loop {
    //     while queue.pending() < 8 {
    //         println!("2");
    //         queue.submit(RequestBuffer::new(256));
    //     }
    //     println!("3");
    //     let result = block_on(queue.next_complete());
    //     println!("{result:?}");
    //     if result.status.is_err() {
    //         break;
    //     }
    // }
}

