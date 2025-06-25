#!/usr/bin/env node

import yargs from 'yargs';
import { hideBin } from 'yargs/helpers';
import { PublicKey, Keypair } from '@solana/web3.js';
import { OnionGill } from '../sdk/index.js';
import dotenv from 'dotenv';
import fs from 'fs';

dotenv.config();

interface DeployTokenArgs {
  network: string;
  payer: string;
  supply: number;
  decimals: number;
}

async function deployToken(args: DeployTokenArgs) {
  try {
    console.log(`🚀 Deploying OnionUSD token on ${args.network}`);
    console.log(`📊 Initial supply: ${args.supply}`);
    console.log(`🔢 Decimals: ${args.decimals}`);

    // Load payer keypair
    let payerKeypair: Keypair;
    if (fs.existsSync(args.payer)) {
      const keypairData = JSON.parse(fs.readFileSync(args.payer, 'utf8'));
      payerKeypair = Keypair.fromSecretKey(new Uint8Array(keypairData));
    } else {
      // Generate new keypair if file doesn't exist
      payerKeypair = Keypair.generate();
      console.log(`⚠️  Generated new payer keypair: ${payerKeypair.publicKey.toString()}`);
    }

    // Connect to cluster
    const builder = OnionGill.connect(args.network as any);

    // Initialize OnionUSD-P configuration
    console.log('⚙️  Initializing OnionUSD-P configuration...');
    const configSig = await OnionGill.initializeConfig(builder, 10, 5); // 10% float, 5% risk
    console.log(`✅ Config initialized: ${configSig}`);

    // Create OnionUSD mint
    console.log('🏭 Creating OnionUSD mint...');
    const mintPubkey = await OnionGill.createMint(builder, args.decimals);
    console.log(`✅ OnionUSD mint created: ${mintPubkey.toString()}`);

    // Create reserve vault
    console.log('🏦 Creating reserve vault...');
    const reserveVault = await OnionGill.createATA(builder, payerKeypair.publicKey, mintPubkey);
    console.log(`✅ Reserve vault created: ${reserveVault.toString()}`);

    // Output deployment info as JSON
    const deploymentInfo = {
      network: args.network,
      onionusd_mint: mintPubkey.toString(),
      reserve_vault: reserveVault.toString(),
      payer: payerKeypair.publicKey.toString(),
      program_id: OnionGill.ONIONUSD_PROGRAM_ID.toString(),
      decimals: args.decimals,
      initial_supply: args.supply,
      timestamp: new Date().toISOString()
    };

    console.log('\n📋 Deployment Summary:');
    console.log(JSON.stringify(deploymentInfo, null, 2));

    // Save deployment info to file
    const outputFile = `deployment-${args.network}-${Date.now()}.json`;
    fs.writeFileSync(outputFile, JSON.stringify(deploymentInfo, null, 2));
    console.log(`💾 Deployment info saved to: ${outputFile}`);

  } catch (error) {
    console.error('❌ Deployment failed:', error);
    process.exit(1);
  }
}

// Configure CLI
const argv = yargs(hideBin(process.argv))
  .command(
    '$0',
    'Deploy OnionUSD token and program',
    (yargs) => {
      return yargs
        .option('network', {
          alias: 'n',
          type: 'string',
          description: 'Solana network to deploy to',
          choices: ['localnet', 'devnet', 'mainnet', 'surfnet'],
          default: 'localnet'
        })
        .option('payer', {
          alias: 'p',
          type: 'string',
          description: 'Path to payer keypair file',
          default: '~/.config/solana/id.json'
        })
        .option('supply', {
          alias: 's',
          type: 'number',
          description: 'Initial token supply',
          default: 1000000
        })
        .option('decimals', {
          alias: 'd',
          type: 'number',
          description: 'Token decimals',
          default: 6
        });
    },
    (argv) => {
      deployToken(argv as DeployTokenArgs);
    }
  )
  .help()
  .alias('help', 'h')
  .parse();