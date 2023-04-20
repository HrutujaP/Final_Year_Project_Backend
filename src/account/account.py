from kybra import query, update,StableBTreeMap,Principal,ic,opt,nat,CanisterResult,Async

from account_structure import Account,generate_id
from src.storage.types import Storage

accounts = StableBTreeMap[Principal,Account](
    memory_id=0,max_key_size=1000,max_value_size=10000
)


storage_canister = Storage(Principal.from_str('ryjl3-tyaaa-aaaaa-aaaba-cai'))

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
def create_storage(Rent:int, OwnerName:str, Path:str, TimePeriod:str, Space:int) ->Async [opt[Principal]]:
    result :CanisterResult[opt[Principal]] =yield storage_canister.postAdvertisement(Rent, OwnerName, Path, TimePeriod, Space)
    
    if result.ok:
        ic.print("Storage created")
        ic.print(result)
        return result.ok
    else:
        ic.print("Storage not created")
        return None
 
@update
def delete_storage(Id:Principal) ->Async [opt[str]]:
    result :CanisterResult[opt[str]] =yield storage_canister.deleteAdvertisement(Id)
    if result:
        ic.print("Storage deleted")
        return result.ok
    else:
        ic.print("Storage not deleted")
        return None
    
@update
def add_rentee(StorageId:Principal, RenterPrincipal:Principal) ->Async [opt[Principal]]:
    result :CanisterResult[opt[Principal]] =yield storage_canister.addRentee(StorageId,RenterPrincipal)
    if result:
        ic.print("Rentee Added!!")
        return result.ok
    else:
        ic.print("Rentee not added")
        return None
    
@update
def remove_rentee(StorageId:Principal) ->Async [opt[Principal]]:
    result :CanisterResult[opt[Principal]] =yield storage_canister.removeRentee(StorageId)
    if result:
        ic.print("Rentee Removed!!")
        return result.ok
    else:
        ic.print("Rentee not removed")
        return None
       
@query
def get_storage(Id:Principal) ->Async [opt[Storage]]:
    result :CanisterResult[opt[Storage]] =yield storage_canister.getAdvertisement(Id)
    if result:
        ic.print("Storage account found")
        return result.ok
    else:
        ic.print("Storage account not found")
        return None

@query
def get_all_accounts() -> list[Principal]:
    return accounts.keys()
    
