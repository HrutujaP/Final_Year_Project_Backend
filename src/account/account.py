from kybra import query, update,StableBTreeMap,Principal,ic,opt,nat,CanisterResult,Async

from account_structure import Account,generate_id,Storages
from src.storage.types import Storage
from src.storage.storage_structure import StorageStruct

accounts = StableBTreeMap[Principal,Account](
    memory_id=0,max_key_size=1000,max_value_size=10000
)


# storage_canister = Storage(Principal.from_str('ryjl3-tyaaa-aaaaa-aaaba-cai'))
storage_canister = Storage(Principal.from_str('iapcx-2yaaa-aaaao-aiz3q-cai'))

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
def add_balance(Id:Principal, amount:nat) -> opt[nat]:
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
def withdraw_balance(Id:Principal, amount:nat) -> opt[nat]:
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
def create_storage(Rent:int, OwnerPrincipal:Principal, Path:str, TimePeriod:str, Space:int) ->Async [opt[StorageStruct]]:
    result :CanisterResult[opt[StorageStruct]] = yield storage_canister.postAdvertisement(Rent, OwnerPrincipal, Path, TimePeriod, Space)
    account = accounts.get(OwnerPrincipal)
    
    if result.ok is not None:
        ic.print(result.ok)
        storage = result.ok
        account["My_Storages"].append(storage["Id"])
        accounts.insert(OwnerPrincipal,account)
        ic.print("Storage created And Added Successfully")
        return result.ok
    else:
        ic.print("Storage not created")
        return result.err
 
@update
def delete_storage(Id:Principal) ->Async [opt[str]]:
    storage :CanisterResult[opt[Storage]] = yield storage_canister.getAdvertisement(Id)
    ic.print(storage)
    if storage.ok is not None: 
        storage = storage.ok
        result :CanisterResult[opt[str]] =yield storage_canister.deleteAdvertisement(Id)
        owner = storage["OwnerPrincipal"]
        renter = storage["RenterPrincipal"]
        
        if renter:
            account = accounts.get(renter)
            storages = account["Rented_Storages"]
            StorageId = str(storage["Id"])
            storages = [i for i in storages if str(i) != StorageId]
            account["Rented_Storages"] = [Principal.from_str(str(i)) for i in storages]
            accounts.insert(renter, account)
        
        if result:
            account = accounts.get(owner)
            storages = account["My_Storages"]
            StorageId = str(storage["Id"])
            storages = [i for i in storages if str(i) != StorageId]
            account["My_Storages"] = [Principal.from_str(str(i)) for i in storages]
            accounts.insert(owner, account)
        
            return result.ok
            
        else:
            ic.print("Storage not deleted")
            return None
        
        
    else:
        ic.print("Storage not found")
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
    result :CanisterResult[opt[str]] =yield storage_canister.removeRentee(StorageId)
    if result:
        try:
            account = accounts.get(RenterPrincipal)
            storages = account["Rented_Storages"]
            StorageId = str(StorageId)
            storages = [i for i in storages if str(i) != StorageId]
            account["Rented_Storages"] = [Principal.from_str(str(i)) for i in storages]
            accounts.insert(RenterPrincipal, account)

        except KeyError:
            return "Rentee not found"
        return result.ok
    else:
        ic.print("Rentee not removed")
        return None
       
@update
def get_storage(Id:Principal) ->Async [opt[StorageStruct]]:
    result :CanisterResult[opt[StorageStruct]] =yield storage_canister.getAdvertisement(Id)
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
    
