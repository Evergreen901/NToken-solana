# Nova finance nToken program


nAssets are Nova Finance’s framework for building programmable assets. nAssets can be used to tokenize and store collective forms of value while also instructing assets to yield, exchange or rebalance. 



Within our Solana MVP, our first nAsset token type allows users to execute a simple hedge where they reduce their exposure to volatility of a cryptocurrency asset by converting and maintaining a % of the nAsset in stablecoin. This nAsset can protect users from downside risk and is a more efficient form of collateral as liquidation is much harder to achieve.



Going forward, we will be releasing portfolios and more complex programmable assets that take profit and redistribute when certain conditions are met. Follow our repository to stay updated!


<img src="https://github.com/NovaFi/NToken-solana/SchemaOfProject.png" style="text-align: center;">



Here a demo from Nova Front end : 


https://user-images.githubusercontent.com/934740/120934721-6b0a7000-c6f7-11eb-987e-2d0dbc734827.mp4





Test our project , follow the next steps
## Social 
- [Website](https://novafinance.app/)
- [Twitter](https://twitter.com/NovaFinance_)
- [Medium](https://novafinance1.medium.com/)
- [Telegram](https://t.me/NovaFinanceGroup)

## Setup Guide 

### install rust 

```bash
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### install solana 

```bash
$ sh -c "$(curl -sSfL https://release.solana.com/v1.6.6/install)"
```


## Download the project 

```bash
$ git clone https://github.com/NovaFi/nToken-solana
```


To run our project , follow the next steps :

First fetch the npm dependencies,  by running:

```bash
$ npm install
```

There are two ways to build the on-chain program :

### first way

```bash
$ cd program
$ cargo build-bpf
```
### second way

```bash
$ cd js 
$ npm run build:program
```
 ## Run the client 

```bash
$ cd js
$ npm install
$ npm run start
```

 ## Test the program 

```bash
$ cd program
$ cargo test
```



