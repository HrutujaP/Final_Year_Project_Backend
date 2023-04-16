from kybra import query, update,StableBTreeMap,Principal,ic,opt,nat
from account.account_structure import Account, generate_id


accounts = StableBTreeMap[Principal,Account](
    memory_id=0,max_key_size=1000,max_value_size=10000
)

balances = StableBTreeMap[Principal,nat](
    memory_id=1,max_key_size=1000,max_value_size=10000
)

balances.insert( Principal.from_str("o35gm-zsefe-wylhy-bq2xh-u53xd-d6xzk-hzwgm-rcs2q-wxmag-ehrjd-kqe"),1000000000)

@update
def create_account(Name:str,email:str) -> opt[Principal]:
    Id = generate_id()
    balances.insert(Id,1000)
    Id = generate_id(email)
    new_account : Account = {
        "Id" : Id,
        "Name" : Name,
        "Email" : email,
        "Balance" : 0
    }
    
    ownerBalance=   balances.get(Principal.from_str("o35gm-zsefe-wylhy-bq2xh-u53xd-d6xzk-hzwgm-rcs2q-wxmag-ehrjd-kqe"))
    balances.insert(Principal.from_str("o35gm-zsefe-wylhy-bq2xh-u53xd-d6xzk-hzwgm-rcs2q-wxmag-ehrjd-kqe"),ownerBalance-1000)
    
    if not accounts.contains_key(Id):
        account = accounts.insert(Id, new_account)
        if account:
            balances.insert(Id,1000)
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
def get_balance(Id:Principal) -> nat:
    # account = accounts.get(Id)
    balance = balances.get(Id)
    # 7w4cb-txlvl-4k3ox-yfgv6-zdddx-qpojm-pe2zb-myj7c-okbqu-yf4l4-ehy
    ic.print(balances.get(Principal.from_str("o35gm-zsefe-wylhy-bq2xh-u53xd-d6xzk-hzwgm-rcs2q-wxmag-ehrjd-kqe")))
    if balance:
        ic.print("Account found")
        return balance
    else:
        ic.print("Account not found")
        return None 

@query
def get_all_accounts() -> list[Principal]:
    keys = accounts.keys()
    return keys

    
