from kybra import Record,Principal
import math
import _random

class Account(Record):
    Id : Principal
    Name : str
    Email : str
    Balance : int


def generate_id(email) -> Principal:
    return Principal.from_str(email)