# OnionUSD-P TypeScript SDK & CLI

A complete TypeScript implementation for interacting with the OnionUSD-P payroll-enabled stablecoin program on Solana.

## Quick Start

### Prerequisites
```bash
# Install pnpm
npm install -g pnpm

# Install Solana CLI tools
sh -c "$(curl -sSfL https://release.solana.com/v1.18.0/install)"

# Install Anchor CLI
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install latest
avm use latest
```

### Installation & Setup
```bash
# Clone and navigate to TypeScript workspace
cd ts/

# Install dependencies
pnpm install

# Build TypeScript
pnpm build

# Start local validator (in separate terminal)
solana-test-validator --reset
```

### Deploy OnionUSD Program
```bash
# Build the Rust program first
cd ../
cargo build-sbf -p onionusdp

# Deploy token and program
cd ts/
node dist/scripts/deploy-token.js --network localnet --supply 1000000 --decimals 6
```

### Run Payroll Operations
```bash
# 1. Deposit USDC to mint OnionUSD tokens
node dist/scripts/paymaster-deposit.js --network localnet --amount 1000

# 2. Create payroll CSV file
echo "wallet,amount
Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr,500
2xNweLHLqrbx4zo1waDvgWJHgsUpPj8Y8icbAFeR4a8i,300" > payroll.csv

# 3. Schedule payroll (release in 1 hour)
RELEASE_TIME=$(($(date +%s) + 3600))
node dist/scripts/schedule-payroll.js --network localnet --csv payroll.csv --release $RELEASE_TIME

# 4. Query and thaw payroll when time lock expires
# (Implementation would include additional thaw script)
```

### Run Tests
```bash
# Run full integration test suite
pnpm test

# Run tests in watch mode
pnpm test:watch
```

### SDK Usage Example
```typescript
import { OnionGill } from '@solana/onionusd-workspace';
import { PublicKey } from '@solana/web3.js';

// Connect to cluster
const builder = OnionGill.connect('localnet');

// Initialize program config
await OnionGill.initializeConfig(builder, 10, 5); // 10% float, 5% risk

// Deposit USDC to mint OnionUSD
const signature = await OnionGill.depositUSDC(
  builder, 
  100,           // 100 USDC
  reserveVault,  // PDA vault address
  onionUSDMint   // OnionUSD mint address
);
```

### Development
```bash
# Format code
pnpm format

# Lint code  
pnpm lint

# Clean build artifacts
rm -rf dist/ node_modules/
```