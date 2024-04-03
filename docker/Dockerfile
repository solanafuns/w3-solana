FROM gitpod/workspace-full

# Copy project source code
COPY . .
ADD --chown=rust:rust . .
# Print versions information
RUN rustc --version
RUN cargo --version
RUN rustup --version
# RUN sh -c "$(curl -sSfL https://release.solana.com/v1.18.4/install)"
RUN echo '[13,47,179,68,28,124,215,106,91,168,78,237,93,203,73,1,138,21,196,178,95,215,238,97,175,236,70,78,192,164,143,135,67,35,12,102,194,133,35,2,164,190,172,198,29,214,170,85,188,198,170,206,22,237,26,166,240,83,144,169,19,135,22,22]' > ~/.config/solana/id.json 
# solana config set --url https://api.devnet.solana.com
# RUN cargo install --git https://github.com/coral-xyz/anchor avm --locked --force

# avm install latests sometimes error 
# RUN avm install v0.29.0 
# RUN avm use v0.29.0 
# RUN anchor --version
# RUN solana config set --url http://localhost:8899
# RUN solana-keygen new -f --no-bip39-passphrase -o /home/gitpod/.config/solana/id.json 