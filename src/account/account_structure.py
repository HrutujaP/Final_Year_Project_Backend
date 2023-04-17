from kybra import Record,Principal,ic
import math
import _random

class Account(Record):
    Id : Principal
    Name : str
    Email : str
    Balance : int


def generate_id(id) -> Principal:
    id = letters(id)
    id = id * 10
    gen_bytes = bytes([ord(c) for c in id[:29]])
    generated_id = Principal.from_hex(gen_bytes.hex())
  
    return generated_id
    
def letters(input):
    return ''.join(filter(str.isalpha, input))