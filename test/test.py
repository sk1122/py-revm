from py_revm import *

# initialise an empty (default) EVM
slot = 8
pool_address = "0x0d4a11d5eeaac28ec3f61d100daf4d40471f1852"
encoded = "0x0902f1ac"

ethersdb = DB(client_url="https://mainnet.infura.io/v3/c60b0bb42f8a4c6481ecd229eddaca27")
acc_info = ethersdb.basic(pool_address)
# query value of storage slot at account address
value = ethersdb.storage(pool_address, slot)

# initialise empty in-memory-db
database = Database()
# insert basic account info which was generated via Web3DB with the corresponding address
database.insert_account_info(pool_address, acc_info)
print(value)
database.insert_account_storage(pool_address, slot, value)

evm = EVM()

# insert pre-built database from above
evm.database(database)

# print(database.accounts)
# print(info.code, info.nonce)

tx_env = RTxEnv.new_call(
    caller = "0x0000000000000000000000000000000000000000",
    transact_to = pool_address,
    data = encoded,
    value = 0
)

env = REnv(
    tx = tx_env
)
evm.set_env(env)

res = evm.transact_ref()
result = res.result.get
print(result)