from kybra import Record,Principal,ic
import math
import _random

class Account(Record):
    Id : Principal
    Name : str
    Email : str
    Balance : int


def generate_id(email) -> Principal:
    generated = Principal.from_str(email)
    ic.print(generated)
    return generated
    
    # random_bytes = bytes(
    #     [math.floor(_random.Random().random() * 256) for _ in range(29)]
    # )
    
    # return Principal.from_hex(random_bytes.hex())
    