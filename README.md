# Awesome Solana Security

Resources to help you build better and more secure Solana programs. Kept up to date.

### Official Resources

- [Solana docs](https://solana.com/docs/) 
- [Solana courses](https://solana.com/developers/courses/)
- [Solana cookbook](https://solana.com/developers/cookbook) 
- [Solana examples supporting multiple frameworks](https://github.com/solana-developers/program-examples)
- [Solana bootcamp 2024](https://github.com/solana-developers/developer-bootcamp-2024)
- [SPL (Solana Program Library) docs](https://spl.solana.com/): SPL is an official collection of programs to help you build your own Solana program.

### Rust
If you need a primer on Rust, these resources are for you:

- [Rust book](https://doc.rust-lang.org/book/)
- [Rust by example](https://doc.rust-lang.org/rust-by-example/index.html)

### Solana Resources Using Native Rust
- [Solana's native Rust docs](https://solana.com/docs/programs/rust)
- [Solana's native Rust course](https://solana.com/developers/courses/native-onchain-development)

### Solana Resources Using Anchor Framework
We highly recommend using [Anchor](https://www.anchor-lang.com), a framework for building secure Solana programs.

- [Anchor docs](https://www.anchor-lang.com/docs)
- [Anchor book](https://book.anchor-lang.com/) 
- [Anchor by example](https://examples.anchor-lang.com/)
- [Anchor wiki](https://docs.rs/anchor-lang)
- [Anchor SPL wiki](https://docs.rs/anchor-spl)

### Solana Resources for EVM transitioners
- [RareSkills' Solana course for Ethereum developers](https://www.rareskills.io/solana-tutorial)
- [0xkowloon's Anchor for EVM developers](https://0xkowloon.gitbook.io/anchor-for-evm-developers)

### General Resources
- [Ackee's Solana handbook](https://ackee.xyz/solana/book/latest/)
- [Blueshift's Anchor and Pinocchio courses and challenges](https://learn.blueshift.gg/)
- [Rektoff's Security roadmap](https://github.com/Rektoff/Security-Roadmap-for-Solana-applications)
- [Helius's blog](https://www.helius.dev/blog): Frequently publishes Solana-related content
- [Pine Analytics's Substack](https://substack.com/@pineanalytics1): Focuses on deep diving into features of Solana protocols
- [Infect3d's Essentials for Starting Solana Auditing](https://www.infect3d.xyz/blog/solana-quick-start)
- [Lucrative_Panda's highly detailed research article that covers all of Solana's security incidents](https://medium.com/@lucrativepanda/a-comprehensive-analysis-of-solanas-security-history-all-incidents-impacts-and-evolution-up-to-1b1564c7ddfe)
- [Solana security course](https://solana.com/developers/courses/program-security)
- [r0bre's 100 Daily Solana Tips](https://accretionxyz.substack.com/p/r0bres-100-daily-solana-tips)
- [Accretion's Hidden IDL Instructions and How to Abuse Them](https://accretionxyz.substack.com/p/hidden-idl-instructions-and-how-to)
- [Farouk ELALEM's explanation of how Solana programs work under the hood](https://ubermensch.blog/under-the-hood-of-solana-program-execution-from-rust-code-to-sbf-bytecode)
- [Ottersec's lamport transfers vulnerabilities](https://osec.io/blog/2025-05-14-king-of-the-sol)
- [Solana security audits: what to do & what not to do w/ Oshield](https://www.youtube.com/watch?v=vw7fqOdf82I): Includes many solana exploited bugs examples 

- Solana's general common vulnerabilities:
  - [Urataps's program examples with vulnerabilities](https://github.com/urataps/solana-audit-examples)
  - [Helius's common vulnerabilities](https://www.helius.dev/blog/a-hitchhikers-guide-to-solana-program-security) 
  - [ImmuneBytes's common Solana attack vectors](https://github.com/ImmuneBytes-Security-Audit/Blockchain-Attack-Vectors/tree/main/Solana%20Attack%20Vectors)
  - [Slowmist's Solana best practices](https://github.com/slowmist/solana-smart-contract-security-best-practices)
  - [Exvul's Solana security guide](https://exvul.com/rust-smart-contract-security-guide-in-solana/)
  - [Zigtur's Solana security walkthrough](https://www.youtube.com/watch?v=xd6qfY-GDYY)
  - [M4rio's Solana security walkthrough](https://www.youtube.com/watch?v=q4z8tIi43lg)
  - [Nirlin's advanced Solana vulnerabilities](https://substack.com/inbox/post/164534668)
  - [Advanced Solana Vulnerabilities w/ r0bre](https://www.youtube.com/watch?v=sStIk5YyWdc)
- Token-2022 Security resources:
  - Offside's Token-2022 best practices [Part 1](https://blog.offside.io/p/token-2022-security-best-practices-part-1) and [Part 2](https://blog.offside.io/p/token-2022-security-best-practices-part-2)
  - [Neodyme's Token-2022 security](https://neodyme.io/en/blog/token-2022)

- [Asymmetric Research's CPI vulnerabilities](https://blog.asymmetric.re/invocation-security-navigating-vulnerabilities-in-solana-cpis/)

### Codebases to Study
**Essential:**
- [Anchor framework](https://github.com/solana-foundation/anchor)
- [Solana system program](https://github.com/solana-program/system)
- [Solana token program](https://github.com/solana-program/token)
- [Solana token-2022 program](https://github.com/solana-program/token-2022)
- [Solana ATA (Associated Token Account) program](https://github.com/solana-program/associated-token-account)
- [Solana token metadata program](https://github.com/solana-program/token-metadata)
- [Metaplex's token metadata program](https://github.com/metaplex-foundation/mpl-token-metadata)

**Optional:**
- [Raydium program](https://github.com/raydium-io/raydium-cp-swap): AMM protocol
- [Kamino program](https://github.com/Kamino-Finance/klend): Lending protocol
- [Squads program](https://github.com/Squads-Protocol/v4): Multisig protocol

### Places to Ask Questions
- [Solana Stack Exchange](https://solana.stackexchange.com/) 

### Tools
- [Solana playground](https://beta.solpg.io/) 
- [Rust playground](https://play.rust-lang.org/)
- [Sec3's IDL Guesser](https://github.com/sec3-service/IDLGuesser): Reverse engineers IDL from onchain programs for easier integration
- [Trail of Bits's Anchor X-ray](https://github.com/crytic/anchorx-ray): Visualizes accounts in Anchor programs
- [John Saigle's Anchor version detector](https://github.com/johnsaigle/anchor-version-detector): Helps figure out which versions of Rust, Solana, and Anchor are compatible with a given Anchor project.

### CTFs
- [Ackee Solana CTF](https://github.com/Ackee-Blockchain/Solana-Auditors-Bootcamp/tree/master/Capture-the-Flag)

### Bug Fix Reviews
- [Raydium - Tick Manipulation Bugfix Review](https://medium.com/immunefi/port-finance-logic-error-bugfix-review-29767aced446](https://medium.com/immunefi/raydium-tick-manipulation-bugfix-review-c6aae4527ed6))
- [Port Finance - Business Logic Error Bugfix Review](https://medium.com/immunefi/port-finance-logic-error-bugfix-review-29767aced446)
- [Post Mortem: ZK ElGamal Proof Program Bug](https://solana.com/tr/news/post-mortem-may-2-2025)

### Hacks, Exploits & Bugs 
- [Solana Hacks, Bugs, and Exploits: A Complete History](https://www.helius.dev/blog/solana-hacks)
- [Loopscale Exploit (Apr 2025)](https://rekt.news/loopscale-rekt)
- [Wormhole Bridge Bug](https://rekt.news/wormhole-rekt)

### Audit Contests
Solana security audits that are publicly available:

- [Orderly on Sherlock](https://audits.sherlock.xyz/contests/524/report): 2 High and 1 Medium 
- [WOOFi on Sherlock](https://audits.sherlock.xyz/contests/535/report): 2 High and 3 Medium 
- [Pump Science on Code4rena](https://code4rena.com/reports/2025-01-pump-science): 2 High and 3 Medium

**NOTE: Contact `0xmorph` in the Cantina Discord server to gain read access if you don't have it.**
- [Grass on Cantina](https://cantina.xyz/competitions/3211ee0d-133f-43a0-837e-8dc1ecfaa424): 13 High and 6 Medium
- [Olas on Cantina](https://cantina.xyz/competitions/829164bf-7fba-4b84-a6b8-76652205bd97): 2 High and 3 Medium
- [Tensor on Cantina](https://cantina.xyz/competitions/21787352-de2c-4a77-af09-cc0a250d1f04): 5 High and 10 Medium
- [ZetaChain on Cantina](https://cantina.xyz/competitions/80a33cf0-ad69-4163-a269-d27756aacb5e): 6 High and 27 Medium (both Solidity and Solana are in-scope)
- [Inclusive Finance on Cantina](https://cantina.xyz/competitions/3eff5a8f-b73a-4cfe-8c54-546b475548f0): 45 High and 25 Medium (both Solidity and Solana are in-scope)
- [Reserve Index on Cantina](https://cantina.xyz/code/8b94becd-54e7-41cd-88e6-caae7becc76a): 10 High and 11 Medium

<!-- 

Pinocchio section:
  

 -->

