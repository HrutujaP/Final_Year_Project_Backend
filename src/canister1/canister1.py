from kybra import query, update

# This is a global variable that is stored on the heap
message: str = ''

# Query calls complete quickly because they do not go through consensus
@query
def get_message() -> str:
    return message

# Update calls take a few seconds to complete
# This is because they persist state changes and go through consensus
@update
def set_message(new_message: str) -> None:
    global message
    message = new_message # This change will be persisted



