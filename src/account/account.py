from kybra import query, update,init,ic,nat8,StableBTreeMap,Principal
from Account import Account,generate_id

accounts:StableBTreeMap[Principal,Account] = StableBTreeMap[Principal,Account](
    memory_id=0,max_key_size=8,max_value_size=1000
)
   
@update
def create_account(Name:str,email:str) -> str:
    Id = generate_id()
    new_account : Account = {
        "Id" : Id,
        "Name" : Name,
        "Email" : email,
        "Balance" : 0
    }
    
    
    if not accounts.contains_key(Id):
        accounts.insert(Id, new_account)
        return "Account created"
    else:
        return "Account already exists"

@update
def delete_account(Id:Principal) -> str:
    if accounts.contains_key(Id):
        accounts.remove(Id)
        return "Account deleted"
    else:
        return "Account does not exist"

@query
def get_account(Id:Principal) -> str:
    name = accounts[Id].name
    return name

@query
def get_balance(Id:int) -> int:
    return accounts[Id].balance

@query
def get_all_accounts() -> list[int]:
    keys = accounts.keys()
    return keys

    
