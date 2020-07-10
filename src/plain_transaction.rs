use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone)]
pub struct PlainData {
    raw: String,
}

#[wasm_bindgen]
impl PlainData {
    pub fn raw(&self) -> String {
        self.raw.clone()
    }
}

#[wasm_bindgen]
pub struct PlainTransaction {
    transactionHash: String,
    format: String,
    sender: String,
    senderType: String,
    recipient: String,
    recipientType: String,
    pub value: u64,
    pub fee: u64,
    pub feePerByte: f64,
    pub validityStartHeight: u32,
    network: String,
    // pub flags: u8,
    data: PlainData,
    proof: PlainData,
    // pub size: u32,
    pub valid: bool,
}

#[wasm_bindgen]
impl PlainTransaction {
    #[wasm_bindgen(method, getter)]
    pub fn transactionHash(&self) -> String {
        self.transactionHash.clone()
    }

    #[wasm_bindgen(method, getter)]
    pub fn format(&self) -> String {
        self.format.clone()
    }

    #[wasm_bindgen(method, getter)]
    pub fn sender(&self) -> String {
        self.sender.clone()
    }

    #[wasm_bindgen(method, getter)]
    pub fn senderType(&self) -> String {
        self.senderType.clone()
    }

    #[wasm_bindgen(method, getter)]
    pub fn recipient(&self) -> String {
        self.recipient.clone()
    }

    #[wasm_bindgen(method, getter)]
    pub fn recipientType(&self) -> String {
        self.recipientType.clone()
    }

    #[wasm_bindgen(method, getter)]
    pub fn network(&self) -> String {
        self.network.clone()
    }

    #[wasm_bindgen(method, getter)]
    pub fn data(&self) -> PlainData {
        self.data.clone()
    }

    #[wasm_bindgen(method, getter)]
    pub fn proof(&self) -> PlainData {
        self.proof.clone()
    }
}
