use bech32;
use clap::Parser;
use secp256k1::{All, Keypair, Secp256k1, SecretKey, schnorr};
use secrecy::{ExposeSecret, SecretString};

#[derive(Parser, Debug)]
pub struct Config {
    #[clap(env = "NOSTR_PRIVATE_KEY")]
    pub key: SecretString,
}

pub struct Signer {
    keypair: Keypair,
}

impl Signer {
    pub fn from_config(config: &Config) -> anyhow::Result<Self> {
        // decode `nsec1...` into Vec<u5>
        let (hrp, decoded) = bech32::decode(&config.key.expose_secret())?;

        if hrp.as_str() != "nsec" {
            anyhow::bail!("expected an nsec key, got prefix {}", hrp);
        }

        // create Vec<u8> from Vec<u5> ("base32")
        // let bytes: Vec<u8> = Vec::from_base32(&decoded).unwrap();
        let bytes: [u8; 32] = decoded
            .try_into()
            .map_err(|_| anyhow::anyhow!("secret key must be exactly 32 bytes"))?;

        // generate a secret key on the curve from the byte vector
        let key = SecretKey::from_secret_bytes(bytes)?;
        let keypair = Keypair::from_secret_key(&key);

        Ok(Self { keypair })
    }

    pub fn public_key(&self) -> String {
        hex::encode(self.keypair.x_only_public_key().0.to_byte_array())
    }

    pub fn sign(&self, id: &str) -> anyhow::Result<String> {
        let digest: [u8; 32] = hex::decode(id)?
            .try_into()
            .map_err(|_| anyhow::anyhow!("event id must be 32 bytes"))?;

        let signature = schnorr::sign_no_aux_rand(&digest, &self.keypair);

        Ok(format!("{signature:x}"))
    }
}
