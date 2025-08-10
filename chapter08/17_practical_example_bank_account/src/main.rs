
pub struct BankAccount {
    account_number: String,
    holder_name: String,
    balance: f64,
    account_type: AccountType,
    transactions: Vec<Transaction>,
}
enum AccountType {
    Checking,
    Savings,
    Investment,
}
struct Transaction {
    date: String,
    amount: f64,
    description: String,
}
impl BankAccount {
    pub fn new(account_number: String, holder_name: String, account_type: AccountType) -> BankAccount {
        BankAccount {
            account_number,
            holder_name,
            balance: 0.0,
            account_type,
            transactions: Vec::new(),
        }
    }
    pub fn deposit(&mut self, amount: f64, description: String) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Deposit amount must be positive".to_string());
        }
        self.balance += amount;
        self.transactions.push(Transaction {
            date: "2023-01-01".to_string(), // In a real app, this would be the current date
            amount,
            description,
        });
        Ok(())
    }
    pub fn withdraw(&mut self, amount: f64, description: String) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Withdrawal amount must be positive".to_string());
        }
        if amount > self.balance {
            return Err("Insufficient funds".to_string());
        }
        self.balance -= amount;
        self.transactions.push(Transaction {
            date: "2023-01-01".to_string(), // In a real app, this would be the current date
            amount: -amount,
            description,
        });
        Ok(())
    }
    pub fn get_balance(&self) -> f64 {
        self.balance
    }
    pub fn get_account_number(&self) -> &str {
        &self.account_number
    }
    pub fn get_holder_name(&self) -> &str {
        &self.holder_name
    }
    pub fn get_transaction_history(&self) -> &[Transaction] {
        &self.transactions
    }
}
