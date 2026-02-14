# 🏦 Kamino Deposit Program

> A Solana program built with Anchor framework that orchestrates complex DeFi operations with the Kamino lending protocol.

---

## 📋 Table of Contents

- [Overview](#overview)
- [Program ID](#program-id)
- [Core Functionality](#core-functionality)
- [Technical Details](#technical-details)
- [Architecture](#architecture)
- [Use Cases](#use-cases)
- [Getting Started](#getting-started)
- [Integration Guide](#integration-guide)
- [Security Considerations](#security-considerations)
- [Important Notes](#important-notes)
- [License](#license)

---

## 🎯 Overview

This program serves as a **CPI (Cross-Program Invocation) wrapper** that executes a complete deposit workflow on Kamino Finance. It chains together multiple Kamino protocol instructions in a single atomic transaction to:

- Initialize lending obligations
- Manage farm rewards
- Deposit collateral
- Update reward accounting

### Key Features

✅ **Atomic Execution** - All operations succeed or fail together  
✅ **Automated Workflow** - 8 complex steps in one transaction  
✅ **Farm Integration** - Automatic liquidity mining setup  
✅ **CPI Safety** - Secure cross-program invocations  

---

## 🆔 Program ID

```
HcxtR55Ec4XQPt47SckxG6RMoWyFAEbSYn5BTmVX7DEE
```

---

## ⚙️ Core Functionality

The program executes **8 sequential operations** in a single transaction:

### Step 0: Initiate Instructions
```
→ Initializes necessary Kamino program instructions
→ Sets up foundation for subsequent operations
```

### Step 1: Initialize Obligation
```
→ Creates new lending obligation account
→ Links obligation to lending market
→ Prepares account structure for collateral deposits
```

### Step 2: Initialize Obligation Farms
```
→ Sets up farming rewards for specific reserve
→ Enables liquidity mining participation
→ Links obligation to farm state accounts
```

### Step 3: Refresh Reserve
```
→ Updates reserve interest rates and state
→ Ensures pricing and accumulator data is current
→ Critical for accurate deposit calculations
```

### Step 4: Refresh Obligation
```
→ Updates obligation state with latest market data
→ Recalculates health factors and positions
```

### Step 5: Refresh Obligation Farms (Pre-Deposit)
```
→ Updates farm reward state before deposit
→ Ensures accurate reward accounting
```

### Step 6: Deposit Reserve Liquidity and Obligation Collateral
```
→ Main deposit operation
→ Transfers user tokens to reserve
→ Mints collateral tokens to obligation
→ Updates all relevant account balances
```

### Step 7: Refresh Obligation Farms (Post-Deposit)
```
→ Updates farm state after deposit
→ Ensures reward tracking reflects new collateral amount
```

---

## 🔧 Technical Details

### Program Structure

```rust
#[program]
pub mod kamino_deposit {
    use anchor_lang::solana_program::program::invoke;
    
    pub fn execute_kamino_operations(
        ctx: Context<ExecuteKaminoOperations>,
        data: Vec<Vec<u8>>,
    ) -> Result<()> {
        // Executes 8 sequential CPI calls to Kamino program
    }
}

#[derive(Accounts)]
pub struct ExecuteKaminoOperations<'info> {
    pub kamino_program: AccountInfo<'info>,
}
```

### Function Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `ctx` | `Context<ExecuteKaminoOperations>` | Program context with Kamino program account info |
| `data` | `Vec<Vec<u8>>` | Vector of 8 instruction data buffers (indices 0-7) |

### Account Requirements

The program expects **68+ accounts** passed as `remaining_accounts`:

| Account Range | Operation | Count |
|---------------|-----------|-------|
| 0-5 | Initiate Instructions | 6 |
| 6-14 | Init Obligation | 9 |
| 15-25 | Init Obligation Farms | 11 |
| 26-31 | Refresh Reserve | 6 |
| 32-33 | Refresh Obligation | 2 |
| 34-43 | Refresh Obligation Farms (Pre) | 10 |
| 44-57 | Deposit Liquidity | 14 |
| 58-67 | Refresh Obligation Farms (Post) | 10 |

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                  Kamino Deposit Program                     │
│                                                             │
│  ┌──────────────────────────────────────────────────────┐  │
│  │  execute_kamino_operations()                         │  │
│  └──────────────────────────────────────────────────────┘  │
│                           │                                 │
│                           ▼                                 │
│         ┌─────────────────────────────────┐                │
│         │   CPI to Kamino Program         │                │
│         └─────────────────────────────────┘                │
│                                                             │
│  Step 0: Initiate          ──────►  invoke()               │
│  Step 1: Init Obligation   ──────►  invoke()               │
│  Step 2: Init Farms        ──────►  invoke()               │
│  Step 3: Refresh Reserve   ──────►  invoke()               │
│  Step 4: Refresh Obligation ─────►  invoke()               │
│  Step 5: Refresh Farms     ──────►  invoke()               │
│  Step 6: Deposit           ──────►  invoke()               │
│  Step 7: Refresh Farms     ──────►  invoke()               │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## 💡 Use Cases

This program is ideal for:

### 1. Automated Deposit Strategies
Execute complex multi-step deposits programmatically without manual intervention.

### 2. DeFi Aggregators
Integrate Kamino deposits into larger protocols and composable strategies.

### 3. Vault Programs
Manage user deposits to Kamino through a vault interface with automated obligation setup.

### 4. Yield Optimizers
Automate deposit and farm initialization in one transaction for optimal capital efficiency.

---

## 🚀 Getting Started

### Prerequisites

- Rust 1.70+
- Solana CLI 1.16+
- Anchor Framework 0.28+

### Installation

```bash
# Clone the repository
git clone <repository-url>
cd kamino-deposit

# Install dependencies
npm install
```

### Building

```bash
# Build the program
anchor build
```

### Testing

```bash
# Run tests
anchor test
```

### Deployment

```bash
# Deploy to devnet
anchor deploy --provider.cluster devnet

# Deploy to mainnet-beta
anchor deploy --provider.cluster mainnet-beta
```

---

## 📚 Integration Guide

### Step 1: Prepare Accounts

```typescript
const accounts = [
  // Accounts 0-5: Initiate
  initiateAccount0,
  initiateAccount1,
  // ... (total 68+ accounts)
];
```

### Step 2: Prepare Instruction Data

```typescript
const instructionData = [
  Buffer.from([/* Step 0 data */]),
  Buffer.from([/* Step 1 data */]),
  // ... (8 total instruction data buffers)
];
```

### Step 3: Call the Program

```typescript
await program.methods
  .executeKaminoOperations(instructionData)
  .accounts({
    kaminoProgram: KAMINO_PROGRAM_ID,
  })
  .remainingAccounts(accounts)
  .rpc();
```

### Step 4: Set Compute Budget

```typescript
const computeBudgetIx = ComputeBudgetProgram.setComputeUnitLimit({
  units: 1_400_000, // Adjust based on needs
});
```

---

## 🔒 Security Considerations

### ✓ Atomic Execution
All operations execute in a single transaction - either all succeed or all fail. No partial states.

### ✓ CPI Safety
Uses standard Solana CPI mechanisms via `invoke()` - no direct account manipulation.

### ✓ Account Validation
Relies on Kamino program's built-in account validation and security checks.

### ✓ Signer Requirements
Multiple accounts require signer status at different steps, enforced by Kamino protocol.

### ⚠️ Risks to Consider

- **Compute Limits**: Transaction may exceed compute budget if not configured properly
- **Account Ordering**: Critical that accounts are in exact order expected
- **Data Format**: Instruction data must be properly serialized for each operation
- **DeFi Risks**: Standard lending protocol risks (liquidation, smart contract risk, etc.)

---

## ⚠️ Important Notes

### Transaction Complexity
This program creates a large transaction with multiple CPI calls. Ensure sufficient compute units are allocated (recommend 1.4M+ compute units).

### Account Ordering
The order of accounts in `remaining_accounts` is **critical** and must match the expected indices exactly. Out-of-order accounts will cause transaction failure.

### Data Format
Each element in the `data` vector must contain properly serialized instruction data for the corresponding Kamino operation. Use Kamino's SDK or instruction builders.

### Compute Budget
Always include compute budget instructions:
```rust
ComputeBudgetProgram.setComputeUnitLimit({ units: 1_400_000 })
ComputeBudgetProgram.setComputeUnitPrice({ microLamports: 1 })
```

---

## 📄 License

[Specify your license]

---

## ⚖️ Disclaimer

This program interacts with the Kamino Finance protocol. Users should:

- Understand the risks associated with DeFi lending protocols
- Review Kamino's documentation and audit reports
- Test thoroughly on devnet before mainnet deployment
- Never deposit more than you can afford to lose
- Conduct your own security audit if integrating into production systems

**This software is provided "as is" without warranty of any kind.**

---

## 🤝 Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Open a Pull Request

---

## 📞 Support

For issues and questions:
- Open an issue on GitHub
- Review Kamino Finance documentation
- Join the Solana developer Discord

---

**Built with ❤️ for the Solana ecosystem**
