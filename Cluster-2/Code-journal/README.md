## Cluster 2 Code journal 
<br>
# Example used - [PDA ANCHOR](https://github.com/solana-developers/program-examples/tree/main/basics/program-derived-addresses/anchor/programs/anchor-program-example/src)
<br>

# What are the concepts (borrowing, ownership, vectors etc)
Program Derived Address

# What is the contract doing? What is the mechanism? 
As i understand, the program implemets a "hash map" of visitors and visit counters. As initialize step, prorgam create state account for every user as a PDA of program_id, user's pubkey ang "page_visits". So every user have it own counter of visits.
And each time user visit a "page" - program increment a counter of "user's" PDA state.