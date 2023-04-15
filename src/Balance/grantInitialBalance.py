from kybra import Principal, StableBTreeMap, nat,query,null


balances = StableBTreeMap[Principal,nat](
    memory_id=0,max_key_size=1000,max_value_size=10000
)



class Token():
    owner = Principal.from_str("o35gm-zsefe-wylhy-bq2xh-u53xd-d6xzk-hzwgm-rcs2q-wxmag-ehrjd-kqe")
    totalSupply = nat = 100000000
    symbol = str = "DSR"
    # balances: hash.__hash__(Principal,nat)(1,Principal.__eq__,Principal.__hash__)

    
    # balances.put(owner,totalSupply);
    balances.insert(owner,totalSupply)
    
    

@query
def balanceOf(who: str) -> nat:

    # who = "o35gm-zsefe-wylhy-bq2xh-u53xd-d6xzk-hzwgm-rcs2q-wxmag-ehrjd-kqe"  
    # let balance : nat = switch balances.__get__(who):
    #     case null 0
    #     case (?result) result;
    # if(balances.__get__(who == null)):
    #     return 0
    # else:
    #     return balances.__get__(who)
    try:
        if(balances.contains_key(Principal.from_str(who))):
            return balances.get(Principal.from_str(who))
    except:
        return 0
    
# def get_principal() -> Principal:
#     return Principal.from_str('rrkah-fqaaa-aaaaa-aaaaq-cai')

# @query
# def print_principal(principal: Principal) -> Principal:
#     ic.print(type(principal))
#     return principal