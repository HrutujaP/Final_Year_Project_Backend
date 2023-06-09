# Important Commands

### To create the virtual environment
```
python3 -m venv venv
```
### To activate the virtual environment
```
source venv/bin/activate
```
### To deactivate the virtual environment
```
deactivate
```

### To shut down wsl
```
wsl --shutdown
```

## To run the project
### 1. Start DFX
```
dfx start --background
```

### 2. Deploy the project
```
dfx deploy (Optional Canister Name)
```

## To Clean Stable Data and clear storages of canister
```
dfx canister uninstall-code (--all to uninstall all canisters / canister_name to uninstall specific canister)

dfx canister uninstall-code --all --network ic
```

## To reinstall the canister
```
dfx canister install --all --network ic
```

## To run the Api
### 1. install node modules
```
npm i
```
### 2. Run the server
```
nodemon
```
## To generate the IDL Factory
```
dfx generate
```

