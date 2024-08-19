# CegaFi Solana Assignment

This project is a smart contract built on the Solana, designed to facilitate limit order trading of tokens. The program allows users to create, update, and manage limit orders, offering flexibility in trading while ensuring security and transparency.

## Features

- **Initialization**: Create a vault for tokens, specifying the amount to sell, price per token, and an expiration time after which the trade cannot occur.
- **TransferToken**: Transfer a specified amount of tokens to the vault created during initialization.
- **Claim**: Check if the expiration time has been reached and, if not, facilitate the trade by transferring tokens to the buyer and sending the equivalent worth of USDC tokens to the seller.
- **Update**: Allows the seller to update the price of each token and modify the expiration time.
- **Close**: Check if partial Trade if not, return the tokens to the seller, close the vault, and return the remaining lamports to the seller.

## How It Works

1. **Initialization**:
  - The seller initializes the order by specifying the details of the trade, including the token amount, price, and expiration time.
  - A vault is created to hold the tokens until the trade is executed or canceled.

2. **TransferToken**:
  - The seller transfers the specified amount of tokens to the vault.

3. **Claim**:
  - The buyer claims the tokens by sending the required amount of USDC to the seller.
  - The tokens are transferred from the vault to the buyer if the expiration time has not been reached.
  - If the expiration time has passed, the trade is no longer valid.

4. **Update**:
  - The seller can update the price per token and the expiration time before the trade is executed.

5. **Close**:
  - If the partial trade does not happen, the seller can close the order, which returns the tokens to the seller, closes the vault, and refunds the lamports.

## Getting Started

### Prerequisites

- Rust
- Solana CLI
- Anchor (for Solana smart contract development)

### Installation

1. Clone the repository:

    ```bash
    git clone https://github.com/AhzamAkhtar/cega-fi-assignment
    ```

2. Build the project:

    ```bash
    anchor build
    ```

3. Deploy the program to the Solana network:

    ```bash
    anchor deploy
    ```
   
### Testing

1. Go To Anchor.toml and put your rpc-url and update your programID
    ```bash
    [registry]
    url = "" //put your devnet rpc url
    ```
   ```
   [programs.devent]
   cega = "" // your program ID
   ```

2. Go To wallet/wallet.ts and put private keys for Seller and Buyer for testing purposes
    ```bash
    export const wallet_for_seller = ""; 
    export const wallet_for_buyer = "";
    ```
3. Go To tests/cega.ts and update the programId and put the mint for token "x" that seller wants to sell
    ```bash
      const programId = new PublicKey(""); // Enter Your ProgramID
                                  &
      // token_x is the token that seller wants to sell                            
      mint_x = new PublicKey(""); // Enter the mint for token_x 
    ```
   
4. Finally Test the Smart Contract by doing
    ```bash
      anchor test
    ```
