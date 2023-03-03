use rand::Rng;



/// Coin tossing protocol based on bit commitment : 
///  -> Given N the lenght of the list we want to shuffle, we generate a random
///     vector of bits of length log(n)

/// Combine the different permutation to agree on one : 
///  -> We receive a vector of random  bits from the different shufllers,
/// 

pub fn rao_sandelius_choose(len: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    if len <= 1 {
        return vec![0]
    }  
    else if len == 2 {
        let r: u8 = rng.gen_range(0..2);
        return vec![r];
    }
    else {
        // Can we accept to only generate a seed to pass through a prng ?
        let mut perm: Vec<u8> = (0..len).map(|_| rng.gen_range(0..2)).collect();
        return perm;
    }
}

/// Permutate the specific slice of our input vector with respect to the given permutation
/// 
/// 
/// # Arguments :
/// 
/// * `in_vec` -> a Vec<u32> input vector to permutate
/// * `perm` -> a Vec<u8> vector of bits defining the permutation 
/// * `start`-> the starting index of the slice to permutate
/// * `end` -> the end index of the slice to permutate
/// 
pub fn rao_sandelius_permutate<T: Copy>(in_vec: &mut Vec<T>, perm: &Vec<u8>, start: usize, end: usize) -> usize {
    if end-start == 1 {
        if perm[0] == 1 {swap(in_vec, start, end);}
        return start;
    }
    else {
        let mut p: usize = start;
        for i in 0..end+1-start {
            if perm[i]<1 {
                swap(in_vec, p, i+start);
                p = p+1;
            } 
        }
        return p;
    }
}

fn swap<T: Copy>(in_vec: &mut Vec<T>, a: usize, b: usize) {
    let tempo: T = in_vec[a];
    in_vec[a] = in_vec[b];
    in_vec[b] = tempo;
}