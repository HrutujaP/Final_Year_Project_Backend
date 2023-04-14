
from kybra import StableBTreeMap,Principal,nat8

balances = StableBTreeMap[Principal,nat8](memory_id=0,max_key_size=8,max_value_size=100)

# To insert into hashmap
balances.insert(Principal.from_text("123456789"), 100)
# Tocheck if key exists in the hashmap
balances.contains_key(Principal.from_text("123456789"))
#  To get value from hashmap
val = balances[Principal.from_text("123456789")]
# To delete from hashmap
balances.remove(Principal.from_text("123456789")) 