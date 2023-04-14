

class Account():
    def __init__(self, balance: int, Name:str,email:str,Id:int):
        self.balance = balance
        self.Name = Name
        self.email = email
        self.Id = Id
        
    def get_balance(self) -> int:
        return self.balance
    