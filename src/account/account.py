from kybra import query, update,init,ic,nat8
from Account import Account


accounts = {}


    
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

    
