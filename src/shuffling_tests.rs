#![allow(unused)]
extern crate replicatedsecretsharing;
use replicatedsecretsharing::*;
use replicatedsecretsharing::datatypes::{Share, ass::*};
use rand::Rng;

use super::shuffling_utils;
use super::debug::debug_util;




pub const W_1: usize = 0;
pub const W_2: usize = 1;
pub const S_1: usize = 2;
pub const S_2: usize = 3;



impl shuffling_utils::Swap for Vec<ASS64> {
    fn swap(&mut self, a: usize, b: usize) {
        let tempo =  self[a];
        self[a] = self[b];
        self[b] = tempo; 
    }
}


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
            init_servers(party, Vec::new(), vec!["127.0.0.1:7777", "127.0.0.1:7778", "127.0.0.1:7779"])
                .expect("Could not initialise Worker 1");
            // generate a randome vector of secret and create to vectors of shares
            let mut rng = rand::thread_rng();
            let mut secret: Vec<u64> = Vec::new();
            let mut secret_shared: Vec<ASS64> = Vec::new();
            for i in 0..15 {
                let s: u64 = rng.gen_range(0..255);
                let share: ASS64 = ASS64::share(s, W_2).expect("Could not share the secret");
                secret.push(s);
                secret_shared.push(share);
            }
            
            // let mut share_1: Vec<u8> = Vec::new();
            // let mut share_2: Vec<u8> = Vec::new();
            // for i in 0..secret.len(){
            //     let (val1,val2): (u8,u8) = generate_shares(secret[i]);
            //     share_1.push(val1);
            //     share_2.push(val2);
            // }
            println!("{:?}", secret);

            test_workers(W_1, secret_shared);
        },
        // Worker 2
        Some(W_2) => {
            init_servers(party, vec!["127.0.0.1:7777"], vec!["127.0.0.1:7780", "127.0.0.1:7781"])
                .expect("Could not initialise Worker 1");

                let mut secret_shared: Vec<ASS64> = Vec::new();
                for i in 0..15 {
                    let share: ASS64 = ASS64::from_share(W_1).expect("Coud not received from W_1");
                    secret_shared.push(share);
                }
                test_workers(W_2, secret_shared);
        },
        // Shuffler 1
        Some(S_1) => {
            init_servers(party, vec!["127.0.0.1:7778", "127.0.0.1:7780"], vec!["127.0.0.1:7782"])
                .expect("Could not initialise Worker 1");
            test_shufflers(S_1, 15);
        },
        // Shuffler 2
        Some(S_2) => {
            init_servers(party, vec!["127.0.0.1:7779", "127.0.0.1:7781", "127.0.0.1:7782"], Vec::new())
                .expect("Could not initialise Worker 1");
            test_shufflers(S_2, 15);
        },
        // Unknown
        Some(_) | None => panic!("unvalid  or none initialized party")
    }
}





pub fn test_workers(party: usize, vect_s: Vec<ASS64> ) -> std::io::Result<()> {
    for s in 0..vect_s.len() {
        vect_s[s].share_to(S_1, S_2)?;
    }

    println!("hey");

    let mut new_vec_s: Vec<ASS64> = Vec::new();
    for _s in 0..vect_s.len() {
        new_vec_s.push(ASS64::from_2_share(S_1, S_2)?);
    }

    let mut res: Vec<u64> = Vec::new();
    for i in 0..new_vec_s.len() {
        res.push(new_vec_s[i].reveal()?)
    }
    assert!(res.len() == vect_s.len());

    println!("finished : {:?}", res);

    Ok(())
}


pub fn test_shufflers(party: usize, len: usize) -> std::io::Result<()> {
    let mut vec_s: Vec<ASS64> = Vec::new();
    for _i in 0..len {
        vec_s.push(ASS64::from_2_share(W_1, W_2)?);
    }

    shuffle(party, &mut vec_s, 0, len);


    for i in 0..len {
        vec_s[i].share_to(W_1, W_2)?;
    }

    Ok(())
}

fn shuffle(party: usize, list: &mut Vec<ASS64>, start: usize, end: usize) {
    println!("start:{:?}; end:{:?}",start, end);
    if end-start > 1{
        let mut perm = shuffling_utils::rao_sandelius_choose(end-start);

        // agrees on permutation
        let mut perm2: Vec<u8> = Vec::new();
        if party == S_1 {
            perm2 = send_and_wait_for_vec(&perm, S_2).expect("Cannot sent or receive from shuffler 2");
        }
        else {
            perm2 = wait_for_vec(S_1).expect("Cannot receive from shuffler 1");
            send_vec_to(S_1, &perm).expect("Cannot send to shuffler 1");
        }
        assert!(perm.len() == perm2.len());
        for i in 0..perm.len() {
            if perm[i] == perm2[i] {perm[i]==0;}
            else {perm[i] = 1;}
        }

        // permutate data
        let p: usize = shuffling_utils::rao_sandelius_permutate(list, &perm, start, end-1);
        //permutate the rest of the list
        if end-start > 2 {
            shuffle(party, list, start, p);
            shuffle(party, list, p, end);
        }
    }
}