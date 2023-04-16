from kybra import query, update,StableBTreeMap,Principal,ic,opt
from Account import Account,generate_id

accounts = StableBTreeMap[Principal,Account](
    memory_id=0,max_key_size=1000,max_value_size=10000
)

   
@update
def create_account(Name:str,email:str) -> opt[Principal]:
    Id = generate_id(email)
    new_account : Account = {
        "Id" : Id,
        "Name" : Name,
        "Email" : email,
        "Balance" : 0
    }
    
    
    if not accounts.contains_key(Id):
        account = accounts.insert(Id, new_account)
        if account:
            ic.print("Account created")
            return Id
        else:
            ic.print("Account not created")
            return None
    else:
        ic.print("Account already exists")
        return None

@update
def delete_account(Id:Principal) -> opt[str]:
    if accounts.contains_key(Id):
        account = accounts.remove(Id)
        if account:
            ic.print("Account deleted")
            return "Account deleted"
        else:
            ic.print("Account not deleted")
            return None
    else:
        ic.print("Account does not exist")
        return None
        

@query
def get_account(Id:Principal) -> opt[str]:
    account = accounts.get(Id)
    if account:
        ic.print("Account found")
        return account["Name"]
    else:
        ic.print("Account not found")
        return None

@query
def get_balance(Id:Principal) -> opt[int]:
    account = accounts.get(Id)
    if account:
        ic.print("Account found")
        return account["Balance"]
    else:
        ic.print("Account not found")
        return None 

@query
def get_all_accounts() -> list[Principal]:
    keys = accounts.keys()
    return keys

    
