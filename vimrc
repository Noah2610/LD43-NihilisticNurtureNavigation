let g:rust_recommended_style = 0

nmap <leader>r :!rm save.json; bin/build && cargo run<CR>
nmap <leader>R :!rm save.json; bin/build && cargo run --release<CR>
