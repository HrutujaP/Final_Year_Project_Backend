# Important Commands

To create the virtual environment
```
python3 -m venv venv
```
To activate the virtual environment
```
source venv/bin/activate
```

1. Start DFX
```
dfx start --background
```

2. Deploy the project
```
dfx deploy (Optional Canister Name)
```

To Clean Stable Data and clear storages of canister
```
dfx canister uninstall-code
```