from kybra import query,update,StableBTreeMap,nat,Principal,ic
from src.account.account import Claim

claimed_accounts = StableBTreeMap[Principal,nat](
    memory_id=1 ,max_key_size=1000,max_value_size=10000
)

claim = Claim(Principal.from_str("ryjl3-tyaaa-aaaaa-aaaba-cai&id=rrkah-fqaaa-aaaaa-aaaaq-cai"))

@update
def claim_Tokens(Id:Principal) -> str:
    if claimed_accounts.contains_key(Id):
        ic.print("Account already claimed")
        return "Account already claimed"
    else:
        claimed_accounts.insert(Id,10000)
        claim.claim_tokens(Id)
        ic.print("Account claimed")
        return "Account claimed"
    
@query
def get_claimed_accounts() -> list[Principal]:
    return claimed_accounts.keys()