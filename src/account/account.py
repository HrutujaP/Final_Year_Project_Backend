from kybra import query, update, StableBTreeMap, Principal, ic, opt, nat
from src.account.structure import Account, generate_id, Storages, StorageStruct


accounts = StableBTreeMap[Principal, Account](
    memory_id=0, max_key_size=1000, max_value_size=10000
)
storages = StableBTreeMap[Principal, StorageStruct](
    memory_id=1, max_key_size=1000, max_value_size=10000
)

available_storages = StableBTreeMap[Principal, StorageStruct](
    memory_id=2, max_key_size=1000, max_value_size=10000
)

# # storage_canister = Storage(Principal.from_str('ryjl3-tyaaa-aaaaa-aaaba-cai'))
# storage_canister = Storage(Principal.from_str('iapcx-2yaaa-aaaao-aiz3q-cai'))


@update
def create_account(Name: str, email: str) -> opt[Principal]:
    # balances.insert(Id,1000)
    Id = generate_id(email+Name)
    new_account: Account = {
        "Id": Id,
        "Name": Name,
        "Email": email,
        "Balance": 100,
        "My_Storages": [],
        "Rented_Storages": []
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
def delete_account(Id: Principal) -> opt[str]:
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
def get_account(Id: Principal) -> opt[Account]:
    account = accounts.get(Id)
    if account:
        ic.print("Account found")
        return account
    else:
        ic.print("Account not found")
        return None


@query
def get_balance(Id: Principal) -> nat:
    account = accounts.get(Id)
    if account:
        ic.print("Account found")
        return account["Balance"]
    else:
        ic.print("Account not found")
        return None


@update
def add_balance(Id: Principal, amount: nat) -> opt[nat]:
    account = accounts.get(Id)
    if account:
        ic.print("Account found")
        account["Balance"] += amount
        accounts.insert(Id, account)
        ic.print("Balance added")
        return account["Balance"]
    else:
        ic.print("Account not found")
        return None


@update
def withdraw_balance(Id: Principal, amount: nat) -> opt[nat]:
    account = accounts.get(Id)
    if account:
        ic.print("Account found")
        if account["Balance"] >= amount:
            account["Balance"] -= amount
            accounts.insert(Id, account)
            ic.print("Balance withdrawn")
            return account["Balance"]
        else:
            ic.print("Not enough balance")
            return account["Balance"]
    else:
        ic.print("Account not found")
        return None


@update
def create_storage(Rent: int, OwnerPrincipal: Principal, TimePeriod: str, Space: int, ndtp:list[str]) -> opt[StorageStruct]:
    adId = generate_id(ndtp[3]+OwnerPrincipal.to_str()+TimePeriod)
    newAdvertisement: StorageStruct = {
        "Id": adId,
        "OwnerPrincipal": OwnerPrincipal,
        "Name": ndtp[0],
        "Description": ndtp[1],
        "Path": ndtp[3],
        "Space": Space,
        "Rent": Rent,
        "TimePeriod": TimePeriod,
        "RenterPrincipal": None,
        "RenteeDuration": None,
        "Timings": ndtp[2],
        "UsedSpace": 0,
        "Files": [],
        "FileExt": [],
    }

    if not storages.contains_key(adId):
        Storage = storages.insert(adId, newAdvertisement)
        if Storage:
            ic.print("Storage created")
            available_storages.insert(adId, newAdvertisement)
            account = accounts.get(OwnerPrincipal)

            if account:
                account["My_Storages"].append(adId)
                accounts.insert(OwnerPrincipal, account)
                ic.print("Storage created And Added Successfully")
                return newAdvertisement
        else:
            ic.print("Storage not created")
            return None
    else:
        ic.print("Storage already exists")
        storage = storages.get(adId)
        return storage


@update
def delete_storage(Id: Principal) -> opt[str]:
    if storages.contains_key(Id):
        storage = storages.remove(Id)

        if storage:
            owner = storage["OwnerPrincipal"]
            renter = storage["RenterPrincipal"]

            owner_account = accounts.get(owner)
            owner_storages = owner_account["My_Storages"]
            owner_storages = [i for i in owner_storages if str(i) != str(Id)]
            owner_account["My_Storages"] = [
                Principal.from_str(str(i)) for i in owner_storages]
            accounts.insert(owner, owner_account)

            if renter:
                renter_account = accounts.get(renter)
                renter_storages = renter_account["Rented_Storages"]
                renter_storages = [
                    i for i in renter_storages if str(i) != str(Id)]
                renter_account["Rented_Storages"] = [
                    Principal.from_str(str(i)) for i in renter_storages]
                accounts.insert(renter, renter_account)

            available_storages.remove(Id)
            ic.print("Storage deleted")
            return "Storage deleted"
        else:
            ic.print("Storage not deleted")
            return None
    else:
        ic.print("Storage does not exist")
        return "Storage does not exist"


@update
def add_rentee(StorageId: Principal, RenterPrincipal: Principal, duration: str) -> opt[Principal]:
    if storages.contains_key(StorageId):
        storage = storages.get(StorageId)
        if storage:
            ic.print("Storage found")
            storage["RenterPrincipal"] = RenterPrincipal
            storage["RenteeDuration"] = duration
            storages.insert(StorageId, storage)
            account = accounts.get(RenterPrincipal)
            account["Rented_Storages"].append(StorageId)
            accounts.insert(RenterPrincipal, account)

            available_storages.remove(StorageId)
            ic.print(storage["RenterPrincipal"])
            return storage["RenterPrincipal"]
    else:
        ic.print("Storage not found")
        return None


@update
def remove_rentee(StorageId: Principal, RenterPrincipal: Principal) -> opt[str]:
    if storages.contains_key(StorageId):
        storage = storages.get(StorageId)
        if storage:
            ic.print("Storage found")
            storage["RenterPrincipal"] = None
            storage["RenteeDuration"] = None
            storages.insert(StorageId, storage)

            account = accounts.get(RenterPrincipal)
            account["Rented_Storages"] = [
                i for i in account["Rented_Storages"] if str(i) != str(StorageId)]
            accounts.insert(RenterPrincipal, account)

            available_storages.insert(StorageId, storage)
            ic.print(storage["RenterPrincipal"])
            return "Rentee removed"
    else:
        ic.print("Storage not found")
        return None


@query
def get_storage(Id: Principal) -> opt[StorageStruct]:
    if storages.contains_key(Id):
        storage = storages.get(Id)
        if storage:
            ic.print("Storage found")
            return storage
    else:
        ic.print("Storage not found")
        return None


@query
def get_all_storages(Id: Principal) -> opt[Storages]:
    account = accounts.get(Id)
    rented_storages_id = account["Rented_Storages"]
    my_storages_id = account["My_Storages"]

    account_storages: Storages = {
        "Rented_Storages": rented_storages_id,
        "My_Storages": my_storages_id
    }

    if account:
        return account_storages
    else:
        return None


@query
def get_available_storages() -> list[StorageStruct]:
    return available_storages.values()


@query
def get_all_accounts() -> list[Principal]:
    return accounts.keys()
