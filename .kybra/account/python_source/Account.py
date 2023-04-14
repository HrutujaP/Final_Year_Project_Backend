from kybra import Record,Principal
import math
import _random

class Account(Record):
    Id : int 
    Name : str
    Email : str
    Balance : int


def generate_id() -> Principal:
    random_bytes = bytes(
        [math.floor(_random.Random().random() * 256) for _ in range(29)]
    )

    return Principal.from_hex(random_bytes.hex())