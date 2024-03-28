


## msg.rs 

- [ ] Create a `NibiruMsg` enum that has fields corresponding to each message in the module. The fields don't need to be grouped by module, and the standard convention is to name the `NibiruMsg::FieldExample` with the corresponding RPC proto method. For example, 
    ```rust
    service Msg {
      rpc RegisterInterchainAccount(MsgRegisterInterchainAccount)
          returns (MsgRegisterInterchainAccountResponse) {};
      rpc SubmitTx(MsgSubmitTx) returns (MsgSubmitTxResponse) {};
    }
    ```

    The interchain account message, `RegisterInterchainAccount`corresponds to the following `NibiruMsg` field:
    ```rust
    pub enum NibiruMsg {
        /// RegisterInterchainAccount registers an interchain account on remote chain.
        RegisterInterchainAccount {
            /// **connection_id** is an IBC connection identifier between Neutron and remote chain.
            connection_id: String,
    
            /// **interchain_account_id** is an identifier of your new interchain account. Can be any string.
            /// This identifier allows contracts to have multiple interchain accounts on remote chains.
            interchain_account_id: String,
    }, 
    ```

- [ ] Write handlers for the `NibiruMsg` fields that convert 

## query.rs

- [ ] Add a `NibiruQuery` enum that has fields corresponding to each query in the module. As with the `NibiruMsg` enum, use field names that match the proto `Query` service of the chain. Generally, all of the custom module queries are added to the same enum rather than split module-wise. I.e., the `x/oracle` and `x/perp` queries should bothbe included on `NibiruQuery`. 

- [ ] Create response structs for each query.
    For example, here's an example from `osmosis/gamm/v2/query.proto`:
    ```proto
    service Query {
      //...
      // SpotPrice defines a gRPC query handler that returns the spot price given
      // a base denomination and a quote denomination.
      rpc SpotPrice(SpotPriceRequest) returns (SpotPriceResponse) {
        option (google.api.http).get =
            "/osmosis/poolmanager/pools/{pool_id}/prices";
      }
    }

    // SpotPriceRequest defines the gRPC request structure for a SpotPrice
    // query.
    message SpotPriceRequest {
      uint64 pool_id = 1 [ (gogoproto.moretags) = "yaml:\"pool_id\"" ];
      string base_asset_denom = 2
          [ (gogoproto.moretags) = "yaml:\"base_asset_denom\"" ];
      string quote_asset_denom = 3
          [ (gogoproto.moretags) = "yaml:\"quote_asset_denom\"" ];
    }
    
    // SpotPriceResponse defines the gRPC response structure for a SpotPrice
    // query.
    message SpotPriceResponse {
      // String of the Dec. Ex) 10.203uatom
      string spot_price = 1 [ (gogoproto.moretags) = "yaml:\"spot_price\"" ];
    }
    ```

    This `SpotPriceResponse` needs a corresponding struct in the bindings contract.
    ```rust
    #[cw_serde]
    pub struct SpotPriceResponse {
        /// How many output we would get for 1 input
        pub price: Decimal,
    }
    ```

    A custom type is used for clarity. 
    ```rust
    #[derive(Serialize, Deserialize, Clone, Eq, PartialEq, JsonSchema, Debug)]
    pub struct Swap {
        pub pool_id: u64,
        pub denom_in: String,
        pub denom_out: String,
    }

    impl Swap {
        pub fn new(pool_id: u64, denom_in: impl Into<String>, denom_out: impl Into<String>) -> Self {
            Swap {
                pool_id,
                denom_in: denom_in.into(),
                denom_out: denom_out.into(),
            }
        }
    }
    ```

    And the `OsmosisQuery` enum includes a corresponding `SpotPrice` field:
    ```rust
    pub enum OsmosisQuery {
        //... 
        #[returns(SpotPriceResponse)]
        SpotPrice { swap: Swap, with_swap_fee: bool },
    }
    ```

    ```rust
    impl OsmosisQuery {
        /// Calculate spot price without swap fee
        pub fn spot_price(pool_id: u64, denom_in: &str, denom_out: &str) -> Self {
            OsmosisQuery::SpotPrice {
                swap: Swap::new(pool_id, denom_in, denom_out),
                with_swap_fee: false,
            }
        }
         
        /// ...
    }
    ```

- [ ] Provide an implementation of the `cosmwasm_std::CustomQuery` trait for the `NibiruQuery` enum.  
    ```rust
    impl CustomQuery for NibiruQuery {}
    ```
    - In the context of CosmWasm, `CustomQuery` is a trait defined in the `cosmwasm_std` crate. Traits in Rust define a set of methods that can be implemented by various types. By implementing the `CustomQuery` trait for `NibiruQuery`, you are specifying that the enum can be used as a custom query type in a CosmWasm smart contract.
    - Inside the curly braces `{}`, you would define the methods required by the `CustomQuery` trait, of which there aren't any.

## TODO.rs

- [ ] TODO


## TODO.rs

- [ ] TODO

# Mapping from protos to Rust 

| Proto | Rust |
| ----  | ---- | 
| `string` | `String` | 
| `repeated string` | `Vec<String>` | 
| `string` | `cosmwasm_std::Decimal` | 
| `sdk.Coin` | `cosmwasm_std::Coin` | 
| `uint64` | `u64` | 

# References

## Wasm Bindings - Rust

- [Neutron-org/neutron-sdk/.../bindings - GitHub](https://github.com/neutron-org/neutron-sdk/tree/4a5fc14e8725ed3fb530e9b97a41abc3cb1e2278/packages/neutron-sdk/src/bindings)
- [Osmosis-labs/bindings - GitHub](https://github.com/osmosis-labs/bindings/tree/v0.7.0/packages/bindings/src)
- [terra-money/terra-cosmwasm - GitHub](https://github.com/terra-money/terra-cosmwasm)
- [CudoVentures/cudos-cosmwasm-bindings - GitHub](https://github.com/CudoVentures/cudos-cosmwasm-bindings/tree/21875435ef3ff985b0e54832e70d50b1af72b6a0/packages/cudos-cosmwasm/src)

## Wasm Bindings - Golang

- [Neutron-org/neutron/wasmbinding - Golang Bindings](https://github.com/neutron-org/neutron/tree/v0.3.1/wasmbinding)
- [Osmosis-labs/osmosis/wasmbinding - Golang Bindings](https://github.com/osmosis-labs/osmosis/tree/v15.0.0/wasmbinding)

## Wasm Bindings - Proto

- [Osmosis-labs/osmosis/proto - GitHub](https://github.com/osmosis-labs/osmosis/tree/v15.0.0/proto/osmosis)
- [Neutron-org/neutron/proto  - GitHub](https://github.com/neutron-org/neutron/tree/v0.3.1/proto)
