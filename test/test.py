from py_revm import *

evm = EVM(env=REnv())

print(dir(evm.transact()))

print(evm.transact()[0])