#![allow(unused)]
extern crate secret_sharing_utils;
use secret_sharing_utils::*;
use rand::Rng;

use super::shuffling_utils;




pub const W_1: usize = 1;
pub const W_2: usize = 2;
pub const S_1: usize = 3;
pub const S_2: usize = 4;


/// set-up the environnment to test the shuffling and sorting algorithm. You need to
/// spawn 4 instances of the program, each running a different entity.
/// We assume a settings with 4 entities: 2 workers, 2 shufflers 
/// By default, we set the following communication channels :
/// W_1 <-> W_2 : 127.0.0.1:7777
/// W_1 <-> S_1 : 127.0.0.1:7778
/// W_1 <-> S_2 : 127.0.0.1:7779
/// w_2 <-> S_1 : 127.0.0.1:7780
/// W_2 <-> S_2 : 127.0.0.1:7781
/// S_1 <-> S_2 : 127.0.0.1:7782
/// 
/// Please note that we use a local external crate "secret_sharing_utils"
/// 
/// # Arguments
/// 
/// * `party` the integer symbolising the entity to spawn
/// 
/// 
/// # Use case :
/// 
/// * it needs to be spawn as follows
/// ```
/// cargo run 1 /* spawns the first worker */
/// cargo run 2 /* spawns the second worker */
/// cargo run 3 /* spawns the first shuffler */
/// cargo run 4 /* spawns the second shuffler */
/// ```
/// 
pub fn run_test(party: Option<usize>) {
    // generate a randome vector of secret and create to vectors of shares

    match party {
        // Worker 1
        Some(W_1) => {
            // generate a randome vector of secret and create to vectors of shares
            let mut rng = rand::thread_rng();
            let secret: Vec<u8> = (0..15).map(|_| rng.gen_range(2..255)).collect();
            let mut share_1: Vec<u8> = Vec::new();
            let mut share_2: Vec<u8> = Vec::new();
            for i in 0..secret.len(){
                let (val1,val2): (u8,u8) = generate_shares(secret[i]);
                share_1.push(val1);
                share_2.push(val2);
            }
            println!("{:?}", secret);
            test_servers(party, Vec::new(), vec!["127.0.0.1:7777", "127.0.0.1:7778", "127.0.0.1:7779"])
                .expect("Could not initialise Worker 1");
            test_workers(1, Some((share_1, share_2))).expect("Worker 1 failed");
        },
        // Worker 2
        Some(W_2) => {
            test_servers(party, vec!["127.0.0.1:7777"], vec!["127.0.0.1:7780", "127.0.0.1:7781"])
                .expect("Could not initialise Worker 1");
            test_workers(2, None).expect("Worker 2 failed");
        },
        // Shuffler 1
        Some(S_1) => {
            test_servers(party, vec!["127.0.0.1:7778", "127.0.0.1:7780"], vec!["127.0.0.1:7782"])
                .expect("Could not initialise Worker 1");
            test_shufflers();
        },
        // Shuffler 2
        Some(S_2) => {
            test_servers(party, vec!["127.0.0.1:7779", "127.0.0.1:7781", "127.0.0.1:7782"], Vec::new())
                .expect("Could not initialise Worker 1");
            test_shufflers();
        },
        // Unknown
        Some(_) | None => panic!("unvalid  or none initialized party")
    }
}



fn generate_shares(val: u8) -> (u8,u8) {
    let mut rng = rand::thread_rng();
    let val1 = val - rng.gen_range(0..val);
    let val2 = val - val1;
    assert!(val == val2 + val1);
    (val1,val2)
}



pub fn test_workers(party: usize, s: Option<(Vec<u8>,Vec<u8>)> ) -> std::io::Result<()>{
    let share: Vec<u8>;
    match s {
        Some((s1,s2)) => {
            assert!(party == W_1);
            share = s1;
            send_vec_to(W_2, &s2).expect("could not send its shares to Worker 2")
        },
        None => {
            assert!(party == W_2);
            share = wait_for_vec(W_1).expect("could not receive the shares for Worker 1");
        }
    }
    let mut share1: Vec<u8> = Vec::new();
    let mut share2: Vec<u8> = Vec::new();
    for i in 0..share.len(){
        let (val1,val2): (u8,u8) = generate_shares(share[i]);
        share1.push(val1);
        share2.push(val2);
    }
    send_vec_to(S_1, &share1).expect("share1 not sent");
    send_vec_to(S_2, &share2).expect("share2 not sent");

    share1 = wait_for_vec(S_1).expect("Cannot receive from Shuffler 1");
    share2 = wait_for_vec(S_2).expect("Cannot receive from Shuffler 2");
    assert!(share1.len() == share2.len());

    let mut received: Vec<u8> = Vec::new();
    for i in 0..share1.len() {
        received.push(share1[i]+share2[i]);
    }

    let mut res: Vec<u8> = Vec::new();
    if party == 1 {
        res = send_and_wait_for_vec(&received, W_2).expect("Cannot sent to Worker 2");
        assert!(res.len() == received.len());
    }
    else {
        res = wait_for_vec(W_1).expect("Worker 2 cannot receieved shares from Worker 1");
        send_vec_to(W_1, &received).expect("Worker 2 cannot send shares to Worker 1");
        assert!(res.len() == received.len());
    }

    for i in 0..received.len() {
        res[i] += received[i];
    }

    println!("{:?}",res);
    Ok(())
}


pub fn test_shufflers() {
    let mut share: Vec<u8> = Vec::new();
    let mut share1: Vec<u8> = wait_for_vec(W_1).expect("Cannot receive from Worker 1");
    let mut share2: Vec<u8> = wait_for_vec(W_2).expect("Cannot receive from Worker 2");
    assert!(share1.len() == share2.len());

    print!("{:?}\n{:?}\n", share1, share2);

    for i in 0..share1.len(){
        share.push(share1[i] + share2[i]);
        let (val1,val2): (u8,u8) = generate_shares(share[i]);
        share1[i] = val1;
        share2[i] = val2;
    }

    print!("{:?}\n{:?}\n", share1, share2);

    send_vec_to(W_1, &share1).expect("Cannot send share back to Worker 1");
    send_vec_to(W_2, &share2).expect("Cannot send share back to Worker 2");
}