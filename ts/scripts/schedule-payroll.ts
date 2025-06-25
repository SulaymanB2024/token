#!/usr/bin/env node

import yargs from 'yargs';
import { hideBin } from 'yargs/helpers';
import { PublicKey, Keypair } from '@solana/web3.js';
import { OnionGill } from '../sdk/index.js';
import dotenv from 'dotenv';
import fs from 'fs';
import csv from 'csv-parser';
import { createReadStream } from 'fs';

dotenv.config();

interface PayrollArgs {
  network: string;
  payer: string;
  csv: string;
  release: number;
}

interface PayrollReceiver {
  wallet: PublicKey;
  amountUi: number;
}

async function parseCSV(csvPath: string): Promise<PayrollReceiver[]> {
  return new Promise((resolve, reject) => {
    const receivers: PayrollReceiver[] = [];
    
    createReadStream(csvPath)
      .pipe(csv())
      .on('data', (row) => {
        try {
          const wallet = new PublicKey(row.wallet || row.address || row.pubkey);
          const amountUi = parseFloat(row.amount || row.value);
          
          if (!wallet || isNaN(amountUi)) {
            throw new Error(`Invalid row: ${JSON.stringify(row)}`);
          }
          
          receivers.push({ wallet, amountUi });
        } catch (error) {
          console.error(`❌ Error parsing CSV row:`, row, error);
        }
      })
      .on('end', () => {
        console.log(`📄 Parsed ${receivers.length} payroll receivers from CSV`);
        resolve(receivers);
      })
      .on('error', reject);
  });
}

async function schedulePayroll(args: PayrollArgs) {
  try {
    console.log(`📅 Scheduling payroll on ${args.network}`);
    console.log(`📄 CSV file: ${args.csv}`);
    console.log(`⏰ Release timestamp: ${args.release} (${new Date(args.release * 1000).toISOString()})`);

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

    // Parse CSV file
    if (!fs.existsSync(args.csv)) {
      console.error(`❌ CSV file not found: ${args.csv}`);
      process.exit(1);
    }

    const receivers = await parseCSV(args.csv);
    if (receivers.length === 0) {
      console.error('❌ No valid receivers found in CSV');
      process.exit(1);
    }

    // Calculate totals
    const totalAmount = receivers.reduce((sum, r) => sum + r.amountUi, 0);
    console.log(`💰 Total payroll amount: ${totalAmount} OnionUSD`);
    console.log(`👥 Recipients: ${receivers.length}`);

    // Validate release time is in the future
    const currentTime = Math.floor(Date.now() / 1000);
    if (args.release <= currentTime) {
      console.error(`❌ Release time must be in the future. Current: ${currentTime}, Release: ${args.release}`);
      process.exit(1);
    }

    // Connect to cluster
    const builder = OnionGill.connect(args.network as any);

    // Generate unique batch ID
    const batchId = BigInt(Date.now());
    console.log(`🏷️  Batch ID: ${batchId}`);

    // Schedule payroll
    console.log('🔄 Scheduling payroll batch...');
    const signature = await OnionGill.schedulePayroll(builder, batchId, receivers, args.release);

    const result = {
      network: args.network,
      operation: 'schedule_payroll',
      batch_id: batchId.toString(),
      total_amount: totalAmount,
      recipient_count: receivers.length,
      release_at: args.release,
      release_date: new Date(args.release * 1000).toISOString(),
      paymaster: payerKeypair.publicKey.toString(),
      signature: signature,
      receivers: receivers.map(r => ({
        wallet: r.wallet.toString(),
        amount: r.amountUi
      })),
      timestamp: new Date().toISOString()
    };

    console.log('\n📋 Payroll Schedule Summary:');
    console.log(JSON.stringify(result, null, 2));

    // Save payroll info
    const outputFile = `payroll-${args.network}-${batchId}.json`;
    fs.writeFileSync(outputFile, JSON.stringify(result, null, 2));
    console.log(`💾 Payroll info saved to: ${outputFile}`);

  } catch (error) {
    console.error('❌ Payroll scheduling failed:', error);
    process.exit(1);
  }
}

// Configure CLI
const argv = yargs(hideBin(process.argv))
  .command(
    '$0',
    'Schedule payroll batch from CSV file',
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
        .option('csv', {
          alias: 'c',
          type: 'string',
          description: 'Path to CSV file with columns: wallet,amount',
          demandOption: true
        })
        .option('release', {
          alias: 'r',
          type: 'number',
          description: 'Unix timestamp when tokens can be released',
          demandOption: true
        });
    },
    (argv) => {
      schedulePayroll(argv as PayrollArgs);
    }
  )
  .help()
  .alias('help', 'h')
  .parse();