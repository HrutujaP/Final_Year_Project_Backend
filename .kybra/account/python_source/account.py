from kybra import query, update,init,ic,nat8

accounts = {}

class Account():
    def __init__(self, balance: int, Name:str,email:str,Id:int):
        self.balance = balance
        self.Name = Name
        self.email = email
        self.Id = Id
        
    def get_balance(self) -> int:
        return self.balance
    
    
@update
def create_account(Name:str,email:str,Id:int) -> str:
    new_account = Account(0, Name, email, Id)
    global accounts
    if not accounts.keys().__contains__(Id):
        accounts[Id] = new_account
        return "Account created"
    else:
        return "Account already exists"

@query
def get_account(Id:int) -> str:
    global accounts
    name = accounts[Id].Name
    return name

@query
def get_balance(Id:int) -> int:
    global accounts
    return accounts[Id].balance

@query
def get_all_accounts() -> str:
    global accounts
    keys = accounts.keys()
    ids = ""
    for key in keys:
        ids += str(key) + " "
    return ids

    
