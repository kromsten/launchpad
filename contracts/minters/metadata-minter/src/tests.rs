mod tests {
    use cosmwasm_std::{testing::mock_dependencies, to_json_binary, StdError};
    use sg_metadata::{Metadata, Trait};
    use rs_merkle::{algorithms::Sha256, Hasher};

    use crate::{helpers::string_to_byte_slice, msg::MintData};
    #[test]
    fn test_inclusion_simple() {

        let _deps = mock_dependencies();

        let merkle_root = String::from("d856e985504d0375ab36b39d783d9491e9f18840f0da07380fb3c8f54945c956");       
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
            token_id: "1".to_string(),
            proof_hashes: vec![
                "66a85f85b71055247d86fe8bae4dc66e81881db74cb52df58489510d757ffafe".to_string(),
                "cb10e2922f2aa7001fc4f6ee4da0c3e8ac162f630166df71d3aef25d719283be".to_string(),
                "e4d853870c6b2dd5b61226e54a0b5bbf15c993299da95904e1f2fcf85c2d8b63".to_string()
            ],
        };

        let init_data_binary = to_json_binary(&mint_data).unwrap();
        let init_data_hash = Sha256::hash(&init_data_binary);

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
        
        assert!(final_hash.is_ok());
        let final_hex = hex::encode(final_hash.unwrap());
        assert_eq!(merkle_root, final_hex);
    }
}