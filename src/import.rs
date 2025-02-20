use crate::utils::{request_new_password, write_wallet_from_mnemonic_and_password};
use anyhow::{bail, Result};
use fuels_signers::WalletUnlocked;
use std::path::Path;

/// Check if given mnemonic is valid by trying to create a `WalletUnlocked` from it
fn check_mnemonic(mnemonic: &str) -> Result<()> {
    // Check users's phrase by trying to create a wallet from it
    if WalletUnlocked::new_from_mnemonic_phrase(mnemonic, None).is_err() {
        bail!("Cannot generate a wallet from provided mnemonics, please check your mnemonic phrase")
    }
    Ok(())
}

pub(crate) fn import_wallet_cli(wallet_path: &Path) -> Result<()> {
    let mnemonic = rpassword::prompt_password("Please enter your mnemonic phrase: ")?;
    check_mnemonic(&mnemonic)?;
    let password = request_new_password();
    write_wallet_from_mnemonic_and_password(wallet_path, &mnemonic, &password)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_utils::TEST_MNEMONIC;

    #[test]
    fn check_mnemonic_should_succeed() {
        assert!(check_mnemonic(TEST_MNEMONIC).is_ok())
    }

    #[test]
    fn check_mnemonic_should_fail() {
        let invalid_mnemonic = "this is an invalid mnemonic";
        assert!(check_mnemonic(invalid_mnemonic).is_err())
    }
}
