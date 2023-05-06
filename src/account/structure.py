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
    RenterPrincipal : opt[Principal]
    Rent : int
    OwnerPrincipal : Principal
    RenteeDuration : opt[str]
    Path : str
    Space: int
    TimePeriod : str

def generate_id(id) -> Principal:
    id = letters(id)
    id = id * 10
    gen_bytes = bytes([ord(c) for c in id[:29]])
    generated_id = Principal.from_hex(gen_bytes.hex())
  
    return generated_id

class Storages(Record):
    My_Storages : list[Principal]
    Rented_Storages : list[Principal]
    
    
def letters(input):
    return ''.join(filter(str.isalpha, input))