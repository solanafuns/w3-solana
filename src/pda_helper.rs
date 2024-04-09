use solana_program::pubkey::Pubkey;

pub struct PdaHelper {
    pub program: Pubkey,
}

impl PdaHelper {
    pub fn new(program: Pubkey) -> Self {
        Self { program }
    }

    fn get_seeds(seed_text: &str) -> Vec<&[u8]> {
        let mut seeds: Vec<&[u8]> = Vec::new();
        if seed_text.len() > 32 {
            for chunk in seed_text.as_bytes().chunks(32) {
                seeds.push(chunk);
            }
        } else {
            seeds.push(seed_text.as_bytes());
        }
        seeds
    }

    pub fn find_program_address_by_text(&self, seed_text: &str) -> (Pubkey, u8) {
        let seeds = Self::get_seeds(seed_text);
        Pubkey::find_program_address(seeds.as_ref(), &self.program)
    }

    pub fn find_program_address_by_text_suffix(
        &self,
        seed_text: &str,
        suffix: &[u8],
    ) -> (Pubkey, u8) {
        let mut seeds = Self::get_seeds(seed_text);
        seeds.push(suffix);
        Pubkey::find_program_address(seeds.as_ref(), &self.program)
    }

    pub fn find_program_address(&self, seeds: &[&[u8]]) -> (Pubkey, u8) {
        Pubkey::find_program_address(seeds, &self.program)
    }
}
