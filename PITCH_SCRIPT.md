# Callit — 3-Minute Stellar Buildathon Pitch Script & Stage Demo Guide
**Hackathon**: Stellar Buildathon (Nigeria & Ghana) 2026  
**Track / Category**: Consumer DeFi / Soroban Smart Contracts / Real-World Adoption  
**Live Smart Contract**: [`CAQG6UE4TJ2HLV6BRMMA5AF3J6ITPUTIGE4YE65YX72C4NRGIOSVXPLR`](https://stellar.expert/explorer/testnet/tx/678106e285e9aa81d7e5b36964595a7706bcae429f44f48ad081f2a5721019ae)  
**Live Arena Companion**: `https://dating-tea-find-mothers.trycloudflare.com/live` (or `callit-web/dist/live/index.html`)

---

## ⏱️ Pitch Timeline (180 Seconds)

### [0:00 - 0:35] Scene 1: The Problem — Why Crypto Prediction Markets Fail in Africa
- **Speaker on Stage**:
  > *"Every weekend across Lagos, Accra, Nairobi, millions of people argue passionately about politics, football, inflation, and music awards. But when they try to use Polymarket or existing crypto prediction markets, they hit two brick walls:*  
  > *First: 24-word seed phrases. 95% of mainstream Africans abandon the app immediately.*  
  > *Second: Gas fees. On Ethereum or Solana, paying $0.40 to $2.00 in rent and gas on a ₦500 micro-bet makes micro-prediction completely uneconomical.*  
  > *What if predicting the future was as fast as FaceID, and cost literally zero gas?"*

---

### [0:35 - 1:15] Scene 2: The Solution — Callit on Stellar Soroban
- **Speaker points to screen**:
  > *"Meet **Callit** — the mobile prediction market engine powered by Stellar Soroban.*  
  > *We solved both friction points natively on Stellar:*  
  > *1. **Native Passkeys (CAP-0051)**: Users sign in using FaceID or TouchID. An authentic Soroban smart account is generated in 3 seconds without a seed phrase.*  
  > *2. **Fee-Bump Sponsorship**: Through Stellar's native fee-bump architecture, Callit's backend sponsors every single transaction. Users pay **$0.00 gas and need 0 XLM**.*  
  > *3. **Instant 3.5s Finality**: Every bet, escrow hold, and claim is settled directly on Soroban smart contracts in under 4 seconds."*

---

### [1:15 - 2:15] Scene 3: The Live Stage Demo (The Crazy Demo)
- **Visual**: Dual-screen split. Left is mobile phone in hand, Right is the projector with the **Live Stage Arena** and a giant QR code.
- **Action**:
  > *"Instead of just showing slides, let's play right now. On the screen is our live Stellar Testnet arena for this showcase:*  
  > ***'Will Callit win 1st Place at the Stellar Buildathon Lagos Showcase?'***  
  > *Right now, judges and everyone in the audience: pull out your phone and point your camera at the QR code on screen.*  
  > *Tap 'FaceID Sign-in' — you don't even need a wallet installed.*  
  > *Tap 'Claim 10 Demo USDC' — our testnet faucet credits you instantly.*  
  > *Now smash that 'BET YES' or 'BET NO' button!"*
- **Visual on Projector**: The live odds progress bar shifts live on stage from 78% YES to 84% YES as judges bet! The on-chain activity feed streams their bets in real time.

---

### [2:15 - 2:45] Scene 4: AI Truth Consensus Engine
- **Speaker**:
  > *"Now, how do markets settle without corrupt or slow human oracles?*  
  > *Callit integrates an **AI Truth Consensus Engine**. When a market reaches its deadline, our consensus engine aggregates real-time news sources, verified APIs, and evidence links, generates a cryptographic SHA-256 digest, and logs it directly into our Soroban smart contract on-chain.*  
  > *Anyone can click 'Verify on StellarExpert' to inspect the immutable resolution proof. It's tamper-proof, transparent, and audited."*

---

### [2:45 - 3:00] Scene 5: Traction, Africa Roadmap & The Close
- **Speaker**:
  > *"Callit is ready today. Our Soroban contract is deployed on Testnet, our Go backend is running on our cloud VPS, our Flutter mobile app connects seamlessly, and our African fiat on-ramps via FossaPay virtual accounts let anyone fund directly in Naira.*  
  > *We are Callit. Where Africa predicts the future, powered by Stellar.*  
  > *Thank you!"*

---

## 🛠️ On-Stage Checklist for Lagos Showcase (Dec 5)
1. **Laptop connected to HDMI Projector**: Open `https://dating-tea-find-mothers.trycloudflare.com/live` in full screen (or local `callit-web/dist/live/index.html`).
2. **Phone in hand**: Have the Callit Flutter app open.
3. **Backup Hotspot**: Enable phone 5G hotspot in case venue Wi-Fi has high latency.
4. **Soroban Faucet Loaded**: Ensure `callit-sponsor` has plenty of testnet XLM for fee sponsorship.
