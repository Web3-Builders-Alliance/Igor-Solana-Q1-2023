## Cluster 1 Code journal 
<br>
# Example used - [CPI NAT LEVER](https://github.com/solana-developers/program-examples/blob/main/basics/cross-program-invocation/native/programs/lever/src/lib.rs)
<br>

# What are the concepts (borrowing, ownership, vectors etc)

# What is the organization?

# What is the contract doing? What is the mechanism? 
As i understand, the contract receives data (power status boolean) and stores it onchain account, that need to be created before. Anf after that, if someone wants to change this status, contract shows info about new status and user who changed it.
Contract uses cross-program invocation to system program while creating state account.

# How could it be better? More efficient? Safer?
We can specify a list of user's accounts who's eligible to make status changes.

# The code could be safer and better if…..
We'll annotate expected accounts in code. And split code to different files accordance with Solana concept