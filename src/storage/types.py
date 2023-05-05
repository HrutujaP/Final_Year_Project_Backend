from kybra import Canister,method,opt,Principal
from src.storage.storage_structure import StorageStruct

class Storage(Canister):
    @method
    def postAdvertisement(self,Rent:int, OwnerPrincipal:Principal, Path:str, TimePeriod:str, Space:int) -> opt[StorageStruct]: ...
    
    @method
    def deleteAdvertisement(self,Id:Principal) -> opt[str]:...
    
    @method 
    def addRentee(self,Id:Principal, RenterPrincipal:Principal,duration:str) -> opt[Principal]:...
    
    @method
    def removeRentee(self,Id:Principal) -> opt[str]:...
    
    @method
    def getAdvertisement(self,Id:Principal) -> opt[StorageStruct]:...
    
    