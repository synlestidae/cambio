import * as React from "react";
import {Account} from './domain/account';
import {CreditCardChoice} from './credit_card_choice';
import {CreditCardInput} from './credit_card_input';
import {CreditOrderInput} from './credit_order_input';
import {ActionCreators} from './flux/action_creators';
import {AccountPage, AccountOption, CreditAccountOption, CreditCardDetails, TransactionListOption} from './flux/state/account_page';
import {CreditCardInfo} from './credit_card_info';
import {TransactionList} from './transaction_list';
import {AccountHeader} from './account_header';

export interface AccountLineProps {
    actions: ActionCreators,
    openOptions: AccountOption,
    account: Account,
    isOpen: boolean
}

export function AccountLine(props: AccountLineProps) {
    let options: any[] = [];
    let isCrediting = false;
    let transactionsOpen = false;
    if (props.openOptions instanceof CreditAccountOption) {
        let ccOptions = props.openOptions as CreditAccountOption;
        let ccDetails = ccOptions.creditCardDetails;
        options = [
            <CreditCardInfo></CreditCardInfo>,
            <CreditCardChoice></CreditCardChoice>,
            <CreditCardInput 
              actions={props.actions}
              cardNumber={ccDetails.cardNumber} 
              expiryMonth={ccDetails.expiryMonth} 
              expiryYear={ccDetails.expiryYear} 
              cvv={ccDetails.cvv}>
            </CreditCardInput>,
            <CreditOrderInput amount={ccOptions.creditDollars} actions={props.actions}>
            </CreditOrderInput>
        ];
        isCrediting = props.isOpen;
    } else if (props.openOptions instanceof TransactionListOption) {
        options = [
            <TransactionList loadingState={props.openOptions.loadingState} transactions={props.openOptions.transactions}>
            </TransactionList>
        ];
        transactionsOpen = props.isOpen;
    }
    const buttonClass = (b: boolean) => `btn ${isCrediting? 'active' : ''}`;
    let toggleCredit = (e: any) => props.actions.toggleCreditAccount(props.account);
    let toggleTransactions = (e: any) => props.actions.toggleTransactions(props.account);
    return (
        <div className="account-container " style={{maxWidth: '500px'}}>
          <AccountHeader actions={props.actions} accountId={"1"}></AccountHeader>
        </div>
    );
}
