from kybra import Record,Principal,opt,ic

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
    id = letters(id) * 29
    ord_list = [ord(c) for c in id[:29]]
    generated_id = Principal.from_hex(bytes(ord_list).hex())
  
    return generated_id

def letters(input):
    return ''.join(filter(str.isalpha, input))
    