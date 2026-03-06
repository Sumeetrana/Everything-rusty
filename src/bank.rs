#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }

    fn deposit(&mut self, amount: i32) {
        self.balance = self.balance + amount;
    }

    fn withdraw(&mut self, amount: i32) {
        self.balance = self.balance - amount;
    }
}

fn main() {
    let mut account = Account::new(1, String::from("me"));
    account.deposit(300);
    account.withdraw(200);
    let mut bank = Bank::new();

    bank.add_account(account);

    println!("{:#?}", bank)
}
