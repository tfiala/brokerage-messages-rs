use super::common::*;
use brokerage_messages::client_messages::*;
use brokerage_messages::domain::*;
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

#[fixture]
pub fn account_ledger_update<'a>()
-> ClientMessage<'a, ClientSubscriptionUpdate<'a, AccountLedgerSubUpdateDetails>> {
    make_account_ledger_update(AccountLedger {
        account_id: ACCOUNT_ID.to_owned(),
        brokerage_id: BROKERAGE_ID.to_owned(),
        account_type: ACCOUNT_TYPE.to_owned(),
        available_funds: AVAILABLE_FUNDS,
        equity_with_loan_value: EQUITY_WITH_LOAN_VALUE,
        gross_position_value: GROSS_POSITION_VALUE,
        cash_balance: CASH_BALANCE,
    })
}
