use usbtmc_message::Sequencer;


fn main() {


    let mut sequencer = Sequencer::new(64);

    let seq = sequencer.command_to_message_sequence("*IDN?");

    for i in 0..seq.len() {
        let message = seq[i].to_vec();
        // SEND 
    }
    
    

}