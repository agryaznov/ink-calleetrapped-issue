> **Solved**: *CalleeTrapped* was caused by *OutOfGas*
> 
> receipt: inspect the callstack to dig into the *trap_reason*
> e.g. by looking for *ExtrinsicFailed* inside block data

# Ink! CalleeTrapped issue demo
I've stuck in getting `CalleeTrapped` error and created this repo for the issue reproduction.  

Please, help to solve it if you have an idea how. 

## Contracts
+ `Flipper` contract is taken from [Ink! examples](https://github.com/paritytech/ink/tree/master/examples)  
  it's `flip()` message is given `0xC0DECAFE` selector 
+ `Caller` contract invokes `Fipper.flip()` message

## Steps to Reproduce
1. Build `Flipper`  
  ```bash
  cd flipper
  cargo +nightly contract build
  ```
2. Build `Caller`  
  ```bash
  cd ../caller
  cargo +nightly contract build
  ```
3. Start node  
  ```bash
   substrate-contracts-node --dev --tmp -lerror,runtime::contracts=debug 
  ```   
4. Open [CanvasUI](https://paritytech.github.io/canvas-ui/#/), upload and instantiate:
   - `Flipper` contract
   - `Caller` contract with `callee_contract_address` = `<Flipper AccountId>`
5. Try to execute `Caller.call_flipper()` as RPC call  
   **Result**: `CalleeTrapped`
5. Try to execute `Caller.call_flipper()` as transaction  
   **Result**: `ContractTrapped`