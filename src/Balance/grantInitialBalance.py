# from kybra import Principal, nat,query,null


# balances = hash()
# class Token():
#     owner : Principal.from_str("o35gm-zsefe-wylhy-bq2xh-u53xd-d6xzk-hzwgm-rcs2q-wxmag-ehrjd-kqe")
#     totalSupply : nat = 100000000
#     symbol : str = "DSR"
#     balances: hash.__hash__(Principal,nat)(1,Principal.__eq__,Principal.__hash__)

    
#     balances.put(owner,totalSupply);
    
    

# @query
# def balanceOf(who: Principal) -> nat:
   
#     # let balance : nat = switch balances.__get__(who):
#     #     case null 0
#     #     case (?result) result;
#     if(balances.__get__(who == null)):
#         return 0
#     else:
#         return balances.__get__(who)
    
# # def get_principal() -> Principal:
# #     return Principal.from_str('rrkah-fqaaa-aaaaa-aaaaq-cai')

# # @query
# # def print_principal(principal: Principal) -> Principal:
# #     ic.print(type(principal))
# #     return principal