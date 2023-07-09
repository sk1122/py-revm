from py_revm import *

# initialise an empty (default) EVM
evm = EVM()

print(Database)

# initialise empty in-memory-db
database = Database()

pool_address = "0x0d4a11d5EEaaC28EC3F61d100daF4d40471f1852"

ethersdb = DB(client_url="https://rpc.ankr.com/eth")
acc_info = ethersdb.basic(pool_address)
# query value of storage slot at account address
value = ethersdb.storage(pool_address, 0)

print(acc_info, value, "acc_info")

info = acc_info

# insert basic account info which was generated via Web3DB with the corresponding address
database.insert_account_info(pool_address, info)
database.insert_account_storage(pool_address, 0, int(value))

# insert pre-built database from above
evm.database(database)

print(database.accounts)
print(info.code, info.nonce)