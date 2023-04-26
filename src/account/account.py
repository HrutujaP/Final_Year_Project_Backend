from kybra import query, update,StableBTreeMap,Principal,ic,opt,nat,CanisterResult,Async

from account_structure import Account,generate_id,Storages
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
        "Balance" : 100,
        "My_Storages" : [],
        "Rented_Storages" : []
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
def get_account(Id:Principal) -> opt[Account]:
    account = accounts.get(Id)
    if account:
        ic.print("Account found")
        return account
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
def add_balance(Id:Principal, amount:nat) -> nat:
    account = accounts.get(Id)
    if account:
        ic.print("Account found")
        account["Balance"] += amount
        accounts.insert(Id,account)
        ic.print("Balance added")
        return account["Balance"]
    else:
        ic.print("Account not found")
        return None

@update
def withdraw_balance(Id:Principal, amount:nat) -> nat:
    account = accounts.get(Id)
    if account:
        ic.print("Account found")
        if account["Balance"] >= amount:
            account["Balance"] -= amount
            accounts.insert(Id,account)
            ic.print("Balance withdrawn")
            return account["Balance"]
        else:
            ic.print("Not enough balance")
            return account["Balance"]
    else:
        ic.print("Account not found")
        return None

@update
def create_storage(Rent:int, OwnerPrincipal:Principal, Path:str, TimePeriod:str, Space:int) ->Async [opt[Storage]]:
    result :CanisterResult[opt[Storage]] = yield storage_canister.postAdvertisement(Rent, OwnerPrincipal, Path, TimePeriod, Space)
    account = accounts.get(OwnerPrincipal)
    if result.ok:
        ic.print(result.ok)
        storage = result.ok
        account["My_Storages"].append(storage["Id"])
        accounts.insert(OwnerPrincipal,account)
        ic.print("Storage created And Added Successfully")
        return result.ok
    else:
        ic.print("Storage not created")
        return None
 
@update
def delete_storage(Id:Principal) ->Async [opt[str]]:
    storage = storage_canister.getAdvertisement(Id)
    result :CanisterResult[opt[str]] =yield storage_canister.deleteAdvertisement(Id)
    owner = storage["OwnerPrincipal"]
    renter = storage["RenterPrincipal"]
    
    if renter:
        account = accounts.get(renter)
        account["Rented_Storages"].remove(Id)
        accounts.insert(renter,account)
    
    account = accounts.get(owner)
    if result:
        account["My_Storages"].remove(Id)
        accounts.insert(owner,account)
        ic.print("Storage deleted")
        return result.ok
    else:
        ic.print("Storage not deleted")
        return None
    
@update
def add_rentee(StorageId:Principal, RenterPrincipal:Principal,duration:str) ->Async [opt[Principal]]:
    result :CanisterResult[opt[Principal]] =yield storage_canister.addRentee(StorageId,RenterPrincipal,duration)
    if result:
        account = accounts.get(RenterPrincipal)
        account["Rented_Storages"].append(StorageId)
        accounts.insert(RenterPrincipal,account)
        ic.print("Rentee Added!!")
        return result.ok
    else:
        ic.print("Rentee not added")
        return None
    
@update
def remove_rentee(StorageId:Principal,RenterPrincipal:Principal) ->Async [opt[str]]:
    result :CanisterResult[opt[Principal]] =yield storage_canister.removeRentee(StorageId)
    if result:
        account = accounts.get(RenterPrincipal)
        account["Rented_Storages"].remove(StorageId)
        accounts.insert(RenterPrincipal,account)
        ic.print("Rentee Removed!!")
        return result.ok
    else:
        ic.print("Rentee not removed")
        return None
       
@query
def get_storage(Id:Principal) ->Async [opt[Storage]]:
    result :CanisterResult[opt[Storage]] =yield storage_canister.getAdvertisement(Id)
    if result.ok:
        ic.print("Storage account found")
        return result.ok
    else:
        ic.print("Storage account not found")
        return None

@query
def get_all_storages(Id:Principal) -> opt[Storages]:
    account = accounts.get(Id)
    rented_storages_id = account["Rented_Storages"]
    my_storages_id = account["My_Storages"]
    
    account_storages : Storages = {
        "Rented_Storages" : rented_storages_id,
        "My_Storages" : my_storages_id
    }
    
    if account:
        return account_storages
    else:
        return None
    

@query
def get_all_accounts() -> list[Principal]:
    return accounts.keys()
    
