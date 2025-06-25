#!/usr/bin/env node

import yargs from 'yargs';
import { hideBin } from 'yargs/helpers';
import { PublicKey, Keypair } from '@solana/web3.js';
import { OnionGill } from '../sdk/index.js';
import dotenv from 'dotenv';
import fs from 'fs';

dotenv.config();

interface DepositArgs {
  network: string;
  payer: string;
  amount: number;
}

async function depositUSDC(args: DepositArgs) {
  try {
    console.log(`💰 Depositing ${args.amount} USDC on ${args.network}`);

    // Load payer keypair
    let payerKeypair: Keypair;
    if (fs.existsSync(args.payer)) {
      const keypairData = JSON.parse(fs.readFileSync(args.payer, 'utf8'));
      payerKeypair = Keypair.fromSecretKey(new Uint8Array(keypairData));
    } else {
      console.error(`❌ Payer keypair file not found: ${args.payer}`);
      process.exit(1);
    }

    console.log(`👤 Paymaster: ${payerKeypair.publicKey.toString()}`);

    // Connect to cluster
    const builder = OnionGill.connect(args.network as any);

    // Load deployment info to get mint and vault addresses
    const deploymentFiles = fs.readdirSync('.')
      .filter(f => f.startsWith(`deployment-${args.network}`) && f.endsWith('.json'))
      .sort()
      .reverse();

    if (deploymentFiles.length === 0) {
      console.error(`❌ No deployment found for ${args.network}. Run deploy-token.ts first.`);
      process.exit(1);
    }

    const deploymentInfo = JSON.parse(fs.readFileSync(deploymentFiles[0], 'utf8'));
    const reserveVault = new PublicKey(deploymentInfo.reserve_vault);
    const onionUSDMint = new PublicKey(deploymentInfo.onionusd_mint);

    console.log(`🏦 Reserve vault: ${reserveVault.toString()}`);
    console.log(`🪙 OnionUSD mint: ${onionUSDMint.toString()}`);

    // Execute deposit
    console.log('🔄 Processing USDC deposit...');
    const signature = await OnionGill.depositUSDC(builder, args.amount, reserveVault, onionUSDMint);

    const result = {
      network: args.network,
      operation: 'deposit_usdc',
      amount: args.amount,
      paymaster: payerKeypair.publicKey.toString(),
      reserve_vault: reserveVault.toString(),
      signature: signature,
      timestamp: new Date().toISOString()
    };

    console.log('\n📋 Deposit Summary:');
    console.log(JSON.stringify(result, null, 2));

    // Save transaction info
    const outputFile = `deposit-${args.network}-${Date.now()}.json`;
    fs.writeFileSync(outputFile, JSON.stringify(result, null, 2));
    console.log(`💾 Transaction saved to: ${outputFile}`);

  } catch (error) {
    console.error('❌ Deposit failed:', error);
    process.exit(1);
  }
}

// Configure CLI
const argv = yargs(hideBin(process.argv))
  .command(
    '$0',
    'Deposit USDC to mint OnionUSD tokens',
    (yargs) => {
      return yargs
        .option('network', {
          alias: 'n',
          type: 'string',
          description: 'Solana network',
          choices: ['localnet', 'devnet', 'mainnet', 'surfnet'],
          default: 'localnet'
        })
        .option('payer', {
          alias: 'p',
          type: 'string',
          description: 'Path to payer keypair file',
          default: '~/.config/solana/id.json'
        })
        .option('amount', {
          alias: 'a',
          type: 'number',
          description: 'Amount of USDC to deposit',
          demandOption: true
        });
    },
    (argv) => {
      depositUSDC(argv as DepositArgs);
    }
  )
  .help()
  .alias('help', 'h')
  .parse();