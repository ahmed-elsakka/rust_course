#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>
}

impl Account {
    fn new(id: u32, holder: String) -> Account {
        Account {
            id,
            holder,
            balance: 0
        }
    }
}

impl Bank {
    fn new()  -> Bank {
        Bank {accounts: vec![]}
    }
}

fn print_account(account: Account) {
    println!("{:#?}", account);
}
fn main() {
    let mut bank = Bank::new();
    let account: Account = Account::new(1, String::from("Ahmed"));
    bank.accounts.push(Account::new(2, String::from("LOL")));

    bank.accounts.iter().for_each(|a| println!("{:#?}", a));
    //print_account(account);

}
