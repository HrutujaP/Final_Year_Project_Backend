from kybra import query, update,StableBTreeMap,Principal,ic,opt,nat

from account_structure import Account,generate_id

accounts = StableBTreeMap[Principal,Account](
    memory_id=0,max_key_size=1000,max_value_size=10000
)

claimed_accounts = StableBTreeMap[Principal,nat](
    memory_id=1 ,max_key_size=1000,max_value_size=10000
)


@update
def create_account(Name:str,email:str) -> opt[Principal]:
    # balances.insert(Id,1000)
    Id = generate_id(email+Name)
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
        return Id

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
def get_balance(Id:Principal) -> nat:
    account = accounts.get(Id)
    if account:
        ic.print("Account found")
        return account["Balance"]
    else:
        ic.print("Account not found")
        return None

@update
def claim_Tokens(Id:Principal) -> str:
    if claimed_accounts.contains_key(Id):
        message = "Account already claimed"
       
    else:
        account = accounts.get(Id)
        if account:
            claimed_accounts.insert(Id,10000)
            account["Balance"] += 10000
            accounts.insert(Id,account)
            message = "Account claimed"
            
        else:
            message = "Invalid account"

    ic.print(message)
    return message


@query
def get_all_accounts() -> list[Principal]:
    return accounts.keys()
    
@query
def get_claimed_accounts() -> list[Principal]:
    return claimed_accounts.keys()