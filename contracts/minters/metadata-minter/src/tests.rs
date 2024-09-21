mod tests {
    use cosmwasm_std::{testing::mock_dependencies, to_json_binary, StdError};
    use sg_metadata::{Metadata, Trait};
    use rs_merkle::{algorithms::Sha256, Hasher};

    use crate::{helpers::string_to_byte_slice, msg::MintData};
    #[test]
    fn test_inclusion_simple() {

        let _deps = mock_dependencies();

        let merkle_root = String::from("8705af05335c87f129954ef218534c081dc9360a29d3ec8e7635a491b3242f76");       
        let mint_data  =  MintData {
            metadata: Metadata {
                name: Some("Space Skellies #1".to_string()),
                description: Some("Were Space Skellies! Protecting the galaxy one planet at a time. Staking, P2E, PVP, and endless rewards for holders!".to_string()),
                image: Some("ipfs://bafybeiavall5udkxkdtdm4djezoxrmfc6o5fn2ug3ymrlvibvwmwydgrkm/1.jpg".to_string()),
                attributes: Some(vec![
                    Trait { trait_type: "Background".to_string(), value: "Eridanus".to_string(), display_type: None },
                    Trait { trait_type: "Body".to_string(), value: "Neon Orange".to_string(), display_type: None },
                    Trait { trait_type: "Apparel".to_string(), value: "None".to_string(), display_type: None },
                    Trait { trait_type: "Mouth Accessories".to_string(), value: "Angry".to_string(), display_type: None },
                    Trait { trait_type: "Eyes Accessories".to_string(), value: "Vr Glasses".to_string(), display_type: None },
                    Trait { trait_type: "Headwear".to_string(), value: "None".to_string(), display_type: None },
                ]),
                ..Default::default()
            },
            token_id: 1u32,
            proof_hashes: vec![
                "dfd8f793aeda2ca3ad9372ad70138cb705f04b7067b414a4a31f8616e87305bd".to_string(),
                "be6f4f58d4d4311c97f0dfa9d59f6759e3fdbb8ce00ae8c5b735c724711c91db".to_string(),
                "3e172bb00cf6a4375726f52db7730559830084d8ad0050e3a61d1a24d91b56f3".to_string()
            ],
        };

        let init_data_binary = to_json_binary(&mint_data).unwrap();
        println!("init data binary: {:?}", init_data_binary.to_base64());
        let init_data_hash = Sha256::hash(&init_data_binary);
        println!("init data hash: {:?}", init_data_hash);
        println!("init data hash hex: {:?}", hex::encode(init_data_hash));

        let final_hash = mint_data.proof_hashes.iter().try_fold(
            init_data_hash,
            |accum_hash_slice, new_proof_hashstring| {
                let mut hashe_slices = [
                    accum_hash_slice,
                    string_to_byte_slice(&new_proof_hashstring)?,
                ];
                hashe_slices.sort_unstable();
                Result::<[u8; 32], StdError>::Ok(Sha256::hash(&hashe_slices.concat()))
            },
        );
        
        println!("final hash: {:?}", final_hash);
        assert!(final_hash.is_ok());
        let final_hex = hex::encode(final_hash.unwrap());
        assert_eq!(merkle_root, final_hex);
    }
}