#![allow(unused)]
extern crate secret_sharing_utils;
use secret_sharing_utils::*;
use rand::Rng;


pub const W_1: usize = 1;
pub const W_2: usize = 2;
pub const S_1: usize = 3;
pub const S_2: usize = 4;



pub fn run_test(party: Option<usize>) {
    /* 
     * we assume 4: 2 workers, 2 shufflers 
     * W_1 <-> W_2 : 127.0.0.1:7777
     * W_1 <-> S_1 : 127.0.0.1:7778
     * W_1 <-> S_2 : 127.0.0.1:7779
     * w_2 <-> S_1 : 127.0.0.1:7780
     * W_2 <-> S_2 : 127.0.0.1:7781
     * S_1 <-> S_2 : 127.0.0.1:7782
     */
    match party {
        // Worker 1
        Some(W_1) => {
            test_servers(party, Vec::new(), vec!["127.0.0.1:7777", "127.0.0.1:7778", "127.0.0.1:7779"])
                .expect("Could not initialise Worker 1");
            //test_workers(1, vec![25,-5,-4]).expect("Worker 1 failed");
        },
        // Worker 2
        Some(W_2) => {
            test_servers(party, vec!["127.0.0.1:7777"], vec!["127.0.0.1:7780", "127.0.0.1:7781"])
                .expect("Could not initialise Worker 1");
            //test_workers(2, vec![-3,9,45]).expect("Worker 2 failed");
        },
        // Shuffler 1
        Some(S_1) => {
            test_servers(party, vec!["127.0.0.1:7778", "127.0.0.1:7780"], vec!["127.0.0.1:7782"])
                .expect("Could not initialise Worker 1");
            //test_shufflers();
        },
        // Shuffler 2
        Some(S_2) => {
            test_servers(party, vec!["127.0.0.1:7779", "127.0.0.1:7781", "127.0.0.1:7782"], Vec::new())
                .expect("Could not initialise Worker 1");
            //test_shufflers();
        },
        // Unknown
        Some(_) | None => panic!("unvalid  or none initialized party")
    }
}






pub fn test_workers(party: usize, share:Vec<i8>) -> std::io::Result<u8>{
    let mut rng = rand::thread_rng();
    for s in 0..share.len() {
        let share1: i8 = share[s] + rng.gen_range(-50..50);
        let share2: i8 = share[s] - share1;
        send_to(S_1, share1).expect("share1 not sent");
        send_to(S_2, share2).expect("share2 not sent");
    }
    let mut received: Vec<i8> = Vec::new();
    for _i in 0..share.len() {
        let share1: i8 = wait_for(S_1).expect("Cannot receive from Shuffler 1");
        let share2: i8 = wait_for(S_2).expect("Cannot receive from Shuffler 2");
        received.push(share1+share2);
    }
    let mut res: Vec<i8> = Vec::new();
    if party == 1 {
        for s in 0..received.len() {
            send_to(W_2, share[s]).expect("Cannot sent to Worker 2");
            let share1: i8 = wait_for(W_2).expect("Cannot receive from Worker 2");
            res.push(received[s]+share1);
        }
    }
    else {
        for s in 0..received.len() {
            let share1: i8 = wait_for(W_1).expect("Cannot receive from Worker 1");
            res.push(received[s]+share1);
            send_to(S_1, share[s]).expect("Cannot send to Worker 1");
        }
    }
    assert!(res == vec![22,4,41]);
    Ok(0)
}
pub fn test_shufflers() {
    // connection to  workers : contact worker ... connect 
    
    // assume id i in [0..N]
    //
    // for j in 0..i do
    //      (2) connect address_j_i 
    // for j in i+1..N do
    //      /!\ we assume workers already now address_i_j, no need to send it
    //      (1) listen on address_i_j

}