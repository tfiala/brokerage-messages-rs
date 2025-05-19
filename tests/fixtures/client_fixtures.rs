// use brokerage_messages::domain::*;
use super::common::*;
use brokerage_messages::client_messages::*;
use rstest::fixture;

#[fixture]
pub fn select_account_response<'a>()
-> ClientMessage<'a, ClientResponse<'a, SelectAccountResponseDetails>> {
    make_select_account_response(
        REQUEST_ID.to_owned(),
        ACCOUNT_ID.to_owned(),
        BROKERAGE_ID.to_owned(),
        true,
    )
}
