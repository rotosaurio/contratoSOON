# Launchpadinsoon

Launchpadinsoon is a token launch program on the Solana network, designed to facilitate the secure and efficient sale and distribution of tokens.

## Features

- Presale initialization
- User whitelist
- Token allocation
- Token purchase
- Vesting system
- Token claiming
- Sale pause and resume
- Presale statistics
- Contract governance

## Project Structure

The project is organized into several modules:

- `lib.rs`: Main entry point of the program
- `presale.rs`: Presale initialization and management logic
- `pricing.rs`: Token purchase logic
- `claim.rs`: Token claiming logic
- `vesting.rs`: Token vesting system
- `sale.rs`: General sale structure and logic
- `governance.rs`: Contract governance functions
- `security.rs`: Security functions such as pause/resume
- `stats.rs`: Functions to obtain presale statistics
- `whitelist.rs`: Whitelist management
- `allocation.rs`: Token allocation management

## Installation

1. Make sure you have Rust and Solana CLI installed.
2. Clone this repository:
   ```
   git clone https://github.com/your-username/launchpadinsoon.git
   ```
3. Navigate to the project directory:
   ```
   cd launchpadinsoon
   ```
4. Build the program:
   ```
   anchor build
   ```

## Usage

To deploy the program on the SOON network:

1. Ensure you have a Solana wallet with sufficient funds.
2. Run:
   ```
   anchor deploy
   ```

## Testing

To run the tests:
anchor test
## Contributing

Contributions are welcome. Please open an issue to discuss major changes before creating a pull request. 


yo could check the deploy here https://explorer.devnet.soo.network/address/bdKtAMn5vzXBGypfiMvPyZy98rwNxuDcUcC2ZrqML2j
