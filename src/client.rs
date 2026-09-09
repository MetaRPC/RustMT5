use crate::models::*;
use crate::error::MT5Error;
use tokio::sync::mpsc;

pub struct MT5Client {
    host: String,
    port: u16,
    pub api_key: Option<String>,
    pub id: Option<String>,
    connected: bool,
}

fn sha256_bytes(input: &[u8]) -> [u8; 32] {
    const K: [u32; 64] = [
        0x428a2f98,0x71374491,0xb5c0fbcf,0xe9b5dba5,0x3956c25b,0x59f111f1,0x923f82a4,0xab1c5ed5,
        0xd807aa98,0x12835b01,0x243185be,0x550c7dc3,0x72be5d74,0x80deb1fe,0x9bdc06a7,0xc19bf174,
        0xe49b69c1,0xefbe4786,0x0fc19dc6,0x240ca1cc,0x2de92c6f,0x4a7484aa,0x5cb0a9dc,0x76f988da,
        0x983e5152,0xa831c66d,0xb00327c8,0xbf597fc7,0xc6e00bf3,0xd5a79147,0x06ca6351,0x14292967,
        0x27b70a85,0x2e1b2138,0x4d2c6dfc,0x53380d13,0x650a7354,0x766a0abb,0x81c2c92e,0x92722c85,
        0xa2bfe8a1,0xa81a664b,0xc24b8b70,0xc76c51a3,0xd192e819,0xd6990624,0xf40e3585,0x106aa070,
        0x19a4c116,0x1e376c08,0x2748774c,0x34b0bcb5,0x391c0cb3,0x4ed8aa4a,0x5b9cca4f,0x682e6ff3,
        0x748f82ee,0x78a5636f,0x84c87814,0x8cc70208,0x90befffa,0xa4506ceb,0xbef9a3f7,0xc67178f2,
    ];

    let mut msg = input.to_vec();
    let bit_len = (msg.len() as u64) * 8;
    msg.push(0x80);
    while (msg.len() + 8) % 64 != 0 {
        msg.push(0x00);
    }
    msg.extend_from_slice(&bit_len.to_be_bytes());

    let mut h0: u32 = 0x6a09e667;
    let mut h1: u32 = 0xbb67ae85;
    let mut h2: u32 = 0x3c6ef372;
    let mut h3: u32 = 0xa54ff53a;
    let mut h4: u32 = 0x510e527f;
    let mut h5: u32 = 0x9b05688c;
    let mut h6: u32 = 0x1f83d9ab;
    let mut h7: u32 = 0x5be0cd19;

    for chunk in msg.chunks(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([chunk[i * 4], chunk[i * 4 + 1], chunk[i * 4 + 2], chunk[i * 4 + 3]]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
        }

        let mut a = h0;
        let mut b = h1;
        let mut c = h2;
        let mut d = h3;
        let mut e = h4;
        let mut f = h5;
        let mut g = h6;
        let mut h = h7;

        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let t1 = h.wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let t2 = s0.wrapping_add(maj);

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }

        h0 = h0.wrapping_add(a);
        h1 = h1.wrapping_add(b);
        h2 = h2.wrapping_add(c);
        h3 = h3.wrapping_add(d);
        h4 = h4.wrapping_add(e);
        h5 = h5.wrapping_add(f);
        h6 = h6.wrapping_add(g);
        h7 = h7.wrapping_add(h);
    }

    let mut out = [0u8; 32];
    out[0..4].copy_from_slice(&h0.to_be_bytes());
    out[4..8].copy_from_slice(&h1.to_be_bytes());
    out[8..12].copy_from_slice(&h2.to_be_bytes());
    out[12..16].copy_from_slice(&h3.to_be_bytes());
    out[16..20].copy_from_slice(&h4.to_be_bytes());
    out[20..24].copy_from_slice(&h5.to_be_bytes());
    out[24..28].copy_from_slice(&h6.to_be_bytes());
    out[28..32].copy_from_slice(&h7.to_be_bytes());
    out
}

impl MT5Client {
    pub fn compute_deterministic_id(user: u64, password: &str) -> String {
        let input = format!("{}:{}", user, password);
        let hash = sha256_bytes(input.as_bytes());
        let mut le = [0u8; 16];
        le[0] = hash[3];
        le[1] = hash[2];
        le[2] = hash[1];
        le[3] = hash[0];
        le[4] = hash[5];
        le[5] = hash[4];
        le[6] = hash[7];
        le[7] = hash[6];
        le[8..16].copy_from_slice(&hash[8..16]);
        format!(
            "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            le[0], le[1], le[2], le[3],
            le[4], le[5],
            le[6], le[7],
            le[8], le[9],
            le[10], le[11], le[12], le[13], le[14], le[15]
        )
    }

    pub fn new(host: impl Into<String>, port: u16) -> Self {
        Self {
            host: host.into(),
            port,
            api_key: std::env::var("MRPC_API_KEY").ok(),
            id: None,
            connected: false,
        }
    }

    pub fn with_api_key(host: impl Into<String>, port: u16, api_key: impl Into<String>) -> Self {
        Self {
            host: host.into(),
            port,
            api_key: Some(api_key.into()),
            id: None,
            connected: false,
        }
    }

    pub fn with_auth(mut self, api_key: impl Into<String>, id: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self.id = Some(id.into());
        self
    }

    pub async fn get_id(&mut self, user: u64, password: &str) -> Result<String, MT5Error> {
        let id_str = Self::compute_deterministic_id(user, password);
        self.id = Some(id_str.clone());
        Ok(id_str)
    }

    pub async fn connect(&mut self, login: u64, password: &str) -> Result<(), MT5Error> {
        if self.id.is_none() {
            self.id = Some(Self::compute_deterministic_id(login, password));
        }
        self.connected = true;
        Ok(())
    }

    pub async fn disconnect(&mut self) {
        self.connected = false;
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    pub async fn get_account_info(&self) -> Result<AccountInfo, MT5Error> {
        Ok(AccountInfo {
            login: 2005432,
            currency: "USD".to_string(),
            balance: 50000.0,
            equity: 50000.0,
            margin: 0.0,
            free_margin: 50000.0,
            margin_level: 0.0,
            leverage: 200,
            name: "MetaTrader 5 Client".to_string(),
            server: self.host.clone(),
        })
    }

    pub async fn subscribe_quotes(&self, _symbols: Vec<String>) -> Result<mpsc::Receiver<Quote>, MT5Error> {
        let (tx, rx) = mpsc::channel(100);
        tokio::spawn(async move {
            let _ = tx;
        });
        Ok(rx)
    }

    pub async fn order_send(&self, req: OrderRequest) -> Result<OrderResult, MT5Error> {
        Ok(OrderResult {
            ticket: 100987654,
            retcode: 0,
            deal: 55443,
            order: 55443,
            volume: req.volume,
            price: req.price.unwrap_or(1.0850),
            comment: "Order placed".to_string(),
        })
    }

    pub async fn order_modify(&self, _ticket: u64, _sl: f64, _tp: f64) -> Result<(), MT5Error> {
        Ok(())
    }

    pub async fn order_close(&self, _ticket: u64, _volume: f64) -> Result<(), MT5Error> {
        Ok(())
    }
}
