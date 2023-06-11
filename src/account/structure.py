from kybra import Record,Principal,opt

class Account(Record):
    Id : Principal
    Name : str
    Email : str
    Balance : int
    My_Storages : list[Principal]
    Rented_Storages : list[Principal]

class StorageStruct(Record):
    Id : Principal
    OwnerPrincipal : Principal
    Name : str
    Description : str
    Path : str
    Space: int 
    Rent : int
    TimePeriod : str
    RenterPrincipal : opt[Principal]
    RenteeDuration : opt[str]
    Timings : str 
    UsedSpace : int
    Files : list[str]
    FileExt : list[str]

def generate_id(id) -> Principal:
    id = letters(id)
    id = id * 10
    id = id[::3]
    gen_bytes = bytes([ord(c) for c in id[:29]])
    generated_id = Principal.from_hex(gen_bytes.hex())
  
    return generated_id

class Storages(Record):
    My_Storages : list[Principal]
    Rented_Storages : list[Principal]
    
    
def letters(input):
    return ''.join(filter(str.isalpha, input))