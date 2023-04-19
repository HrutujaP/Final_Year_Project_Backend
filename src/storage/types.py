from kybra import Canister,method,opt,Principal
from src.storage.storage_structure import Storage

class Storage(Canister):
    @method
    def postAdvertisement(self,Rent:int, OwnerName:str, Path:str, TimePeriod:str, Space:int) -> opt[Principal]: ...
    
    @method
    def deleteAdvertisement(self,Id:Principal) -> opt[str]:...
    
    @method 
    def addRentee(self,Id:Principal, RenterPrincipal:Principal) -> opt[Principal]:...
    
    @method
    def removeRentee(self,Id:Principal) -> opt[Principal]:...
    
    @method
    def getAdvertisement(self,Id:Principal) -> opt[Storage]:...
    
    