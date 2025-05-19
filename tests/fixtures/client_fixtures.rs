use super::common::*;
use brokerage_messages::client_messages::*;
use brokerage_messages::domain::*;
use rstest::fixture;

//
// Responses
//

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
pub fn eod_summary_response<'a>()
-> ClientMessage<'a, ClientResponse<'a, EndOfDaySummaryResponseDetails>> {
    make_end_of_day_summary_response::<'a>(
        REQUEST_ID.to_owned(),
        EndOfDaySummary {
            account_id: ACCOUNT_ID.to_owned(),
            brokerage_id: BROKERAGE_ID.to_owned(),
            net_cash_balance: NET_CASH_BALANCE,
            net_stock_balance: NET_STOCK_BALANCE,
            interest: INTEREST,
            interest_mtd: INTEREST_MTD,
            dividends: DIVIDENDS,
            dividends_mtd: DIVIDENDS_MTD,
            commissions: COMMISSIONS,
            commissions_mtd: COMMISSIONS_MTD,
            other_fees: OTHER_FEES,
            other_fees_mtd: OTHER_FEES_MTD,
        },
    )
}

//
// Subscription Updates
//

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
