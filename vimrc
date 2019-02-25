let g:rust_recommended_style = 0

nmap <leader>r :!bin/build && cargo run<CR>
nmap <leader>R :!bin/build && cargo run --release<CR>
