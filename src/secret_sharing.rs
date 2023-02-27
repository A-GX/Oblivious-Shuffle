
use rand::Rng;


// WE NEED ADDITIVE SECRET SHARING => SHAMIR WON'T DO
pub struct Share {
    bytes: u8
}

impl Share {
    pub fn from_list_share(shares: &Vec<Share>) -> Result<Share>{
        let res: Share = Share {bytes: 0};
        for s in shares {
            res.bytes += s.bytes;
        }
        Ok(res)
    }
    pub fn to_list_share(&self, len: usize) -> Result<Vec<Share>> {
        let mut rng = rand::thread_rng();
        let res: Vec<Share> = Vec::new();
        let mut tot = self.bytes;
        for i in 0..len-1 {
            let r = gen_range(0..tot);
            tot -= r;
            res.push(Share {bytes: r});
        }
        res.push(Share {bytes: tot});
        Ok(res)
    }
}