use super::Share;

#[derive(Debug)]
struct Party {
    id: u32,
    address: u32,
    shares: Vec<Share>
}


struct Workers {
    list: Vec<Party>
}


pub trait ShareSecret<Parties> {
    pub fn send_shares_to(&self, p: Parties) -> Result<>;
    fn get_from_network(&self, p: Parties) -> Result<>;
}

impl ShareSecret<Workers> for Workers {
    fn send_shares_to(&self, p: Shufflers) -> Result<> {
        let nb_shufllers: usize = p.list.len();
        let mut vec: Vec<Vec<Share>> = vec![Vec::new(); nb_shufllers];
        for s in self.shares {
            let list: Vec<Share> = s.to_list_share(nb_shufllers)?;
            for i in 0..nb_shufllers {
                vec[i].push(list[i]);
            }
        }
        for i in 0..nb_shufllers {
            Share::send_to(vec[i], p.list[i].address)?;
        }
        Ok()
    }
    fn get_from_network(&self, p: Parties) -> Result<>;
}