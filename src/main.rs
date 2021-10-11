use clap::Clap;
use std::io::{self, BufRead, Write};
use std::fs::{File, write, OpenOptions};
use std::path::Path;
use crate::utils::{read_lines, to_days, human, to_seconds};
use std::str::FromStr;
use std::num::ParseIntError;
use crate::near::lockup_contract::VestingInformation;

mod utils;
mod near;

#[derive(Clap, Debug, Clone)]
#[clap(version = "0.2.0", author = "Near Inc. <hello@nearprotocol.com>")]
pub(crate) struct Opts {
    #[clap(short, long)]
    pub lockup_account_id: String,
    #[clap(short)]
    pub block_height: Option<u64>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let block = near::rpc::get_block(None).await?;
        let filename = "lockups.txt";
        let res_filename = "res.txt";
        let mut res_file = File::create(res_filename).expect("Can't create file");

        for (i, line) in read_lines(filename).enumerate() {
            let account_id = line.expect("Can't read line");

            // data source. All that we have
            let state = near::rpc::get_account_state(account_id.to_string(), block.height)
                .await?
                .expect(&("Can't get account details for ".to_owned() + &account_id));

            // The amount of tokens locked for this account at the moment of deploying the contract.
            let lockup_amount = state.lockup_information.lockup_amount;

            // The amount of tokens that were withdrawn by NEAR foundation due to early termination
            // of vesting.
            // we are interested in it just to check do we need to take in into account
            let termination = state.lockup_information.termination_withdrawn_tokens;

            // We can't be sure in start of the contract, that's the main issue

            // Size of cliff
            let lockup_duration = state.lockup_information.lockup_duration;

            // Size of lockup period after cliff
            let release_duration = state.lockup_information.release_duration;
            let human_release_duration = match release_duration {
                None => { "empty_release_duration".to_string() }
                Some(x) => { x.to_string() }
            };

            // Date when cliff ends. Could be none, unfortunately
            // That could be our chance to detect start of contract
            let lockup_timestamp = state.lockup_information.lockup_timestamp;
            let human_lockup_timestamp = match lockup_timestamp {
                None => { "empty_lockup_timestamp".to_string() }
                Some(x) => { x.to_string() }
            };

            // Contains all that we need, but it could be none or hashed
            // UPD: decided not to use vesting info
            let vesting_info = match state.vesting_information {
                VestingInformation::None => { "empty_vesting_info".to_string() }
                VestingInformation::VestingHash(_) => { "vesting_hash".to_string() }
                VestingInformation::VestingSchedule(ref s) => {
                    format!("vesting_schedule({};{};{})",
                            s.start_timestamp.0, s.cliff_timestamp.0, s.end_timestamp.0).to_string()
                }
                VestingInformation::Terminating(ref t) => {
                    format!("vesting_terminating({};{:?})", t.unvested_amount.0, t.status)
                }
            };

            let locked_amount = state.get_locked_amount(block.timestamp).0;

            let a = format!("{},{},{},{},{}\n",
                            account_id,
                            state.owner_account_id,
                            lockup_amount,
                            vesting_info,
                            termination,
            );
            if lockup_amount > u128::pow(10, 24) * 10_000 {
                res_file.write(a.as_bytes());
            }

            if i % 100 == 0 {
                println!("{} ", i)
            }
        }

    Ok(())
}
