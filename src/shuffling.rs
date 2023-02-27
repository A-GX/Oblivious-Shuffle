use super::party::{ShareSecret};

pub fn shuffle() -> Result<String> {
    Workers::share_secret(Shufflers)?;
    permute(Shufflers)?;
    Shufflers::share_secret(Workers)?;

    Ok(String::from("Successfully shuffled"))
}