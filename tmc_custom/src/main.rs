use usbtmc_message::Sequencer;
use usbtmc_message::BulkInMessage;

fn main()
{
    // Create a sequencer with a max_sequence_length of 64 (depend on your device)
    let mut sequencer = Sequencer::new(64);

    // Create a message sequence from a command
    let seq = sequencer.command_to_message_sequence("*IDN?");

    // Send the sequence on the usb
    for i in 0..seq.len() {
        let message = seq[i].to_vec();
        // SEND TO USB
    }
    
    // RECEIVE FROM USB
    let data = vec![
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x05, 0x06, 0x07, 0x08,
        0x48, 0x45, 0x4C, 0x4C, 0x4F, 13
    ];

    // Parse the received data
    let msg = BulkInMessage::from_u8_array(&data);

    // Check the message
    assert_eq!( msg.payload_as_string(), "HELLO".to_string() );
}