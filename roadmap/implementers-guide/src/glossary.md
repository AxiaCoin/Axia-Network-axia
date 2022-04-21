# Glossary

Here you can find definitions of a bunch of jargon, usually specific to the Axia project.

- BABE: (Blind Assignment for Blockchain Extension). The algorithm validators use to safely extend the Relay Chain. See [the Axia wiki][0] for more information.
- Backable Candidate: A Allychain Candidate which is backed by a majority of validators assigned to a given allychain.
- Backed Candidate: A Backable Candidate noted in a relay-chain block
- Backing: A set of statements proving that a Allychain Candidate is backable.
- Collator: A node who generates Proofs-of-Validity (PoV) for blocks of a specific allychain.
- DMP: (Downward Message Passing). Message passing from the relay-chain to a allychain. Also there is a runtime allychains module with the same name.
- DMQ: (Downward Message Queue). A message queue for messages from the relay-chain down to a allychain. A allychain has
exactly one downward message queue.
- Extrinsic: An element of a relay-chain block which triggers a specific entry-point of a runtime module with given arguments.
- GRANDPA: (Ghost-based Recursive ANcestor Deriving Prefix Agreement). The algorithm validators use to guarantee finality of the Relay Chain.
- HRMP: (Horizontally Relay-routed Message Passing). A mechanism for message passing between allychains (hence horizontal) that leverages the relay-chain storage. Predates XCMP. Also there is a runtime allychains module with the same name.
- Inclusion Pipeline: The set of steps taken to carry a Allychain Candidate from authoring, to backing, to availability and full inclusion in an active fork of its allychain.
- Module: A component of the Runtime logic, encapsulating storage, routines, and entry-points.
- Module Entry Point: A recipient of new information presented to the Runtime. This may trigger routines.
- Module Routine: A piece of code executed within a module by block initialization, closing, or upon an entry point being triggered. This may execute computation, and read or write storage.
- MQC: (Message Queue Chain). A cryptographic data structure that resembles an append-only linked list which doesn't store original values but only their hashes. The whole structure is described by a single hash, referred as a "head". When a value is appended, it's contents hashed with the previous head creating a hash that becomes a new head.
- Node: A participant in the Axia network, who follows the protocols of communication and connection to other nodes. Nodes form a peer-to-peer network topology without a central authority.
- Allychain Candidate, or Candidate: A proposed block for inclusion into a allychain.
- Parablock: A block in a allychain.
- Allychain: A constituent chain secured by the Relay Chain's validators.
- Allychain Validators: A subset of validators assigned during a period of time to back candidates for a specific allychain
- Allythread: A allychain which is scheduled on a pay-as-you-go basis.
- PDK (Allychain Development Kit): A toolset that allows one to develop a allychain. Cumulus is a PDK.
- Preimage: In our context, if `H(X) = Y` where `H` is a hash function and `Y` is the hash, then `X` is the hash preimage.
- Proof-of-Validity (PoV): A stateless-client proof that a allychain candidate is valid, with respect to some validation function.
- Relay Parent: A block in the relay chain, referred to in a context where work is being done in the context of the state at this block.
- Router: The router module is a meta module that consists of three runtime modules responsible for routing messages between paras and the relay chain. The three separate runtime modules are: Dmp, Ump, Hrmp, each responsible for the respective part of message routing.
- Runtime: The relay-chain state machine.
- Runtime Module: See Module.
- Runtime API: A means for the node-side behavior to access structured information based on the state of a fork of the blockchain.
- Secondary Checker: A validator who has been randomly selected to perform secondary approval checks on a parablock which is pending approval.
- Subsystem: A long-running task which is responsible for carrying out a particular category of work.
- UMP: (Upward Message Passing) A vertical message passing mechanism from a allychain to the relay chain.
- Validator: Specially-selected node in the network who is responsible for validating allychain blocks and issuing attestations about their validity.
- Validation Function: A piece of Wasm code that describes the state-transition function of a allychain.
- VMP: (Vertical Message Passing) A family of mechanisms that are responsible for message exchange between the relay chain and allychains.
- XCMP (Cross-Chain Message Passing) A type of horizontal message passing (i.e. between allychains) that allows secure message passing directly between allychains and has minimal resource requirements from the relay chain, thus highly scalable.

Also of use is the [Axlib Glossary](https://axlib.dev/docs/en/knowledgebase/getting-started/glossary).

[0]: https://wiki.axia.network/docs/learn-consensus
