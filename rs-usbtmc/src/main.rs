use rs_usbtmc::UsbtmcClient;

const DEVICE_VID: u16 = 0x1313;
const DEVICE_PID: u16 = 0x8079;

fn main() {
    // connect to the device
    let device = UsbtmcClient::connect(DEVICE_VID, DEVICE_PID).expect("failed to connect");

    // send a command to the device
    device.command("*IDN?").expect("failed to send command");

    // query the device and get a string
    let response: String = device.query("*IDN?").expect("failed to query device");

    // query the device and get a bytes
    let response: Vec<u8> = device.query_raw("*IDN?").expect("failed to query device");
}