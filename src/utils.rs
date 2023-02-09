use crate::models::Client;

pub fn which_client(client_id: usize) -> Client {
    match client_id {
        0 => Client::Desktop,
        1 => Client::Mobile,
        _ => Client::Unknown,
    }
}
