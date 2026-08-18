#!/usr/bin/env bash

set -Eeuo pipefail

readonly SCRIPT_NAME="$(basename "$0")"
readonly MANAGED_MARKER='" Managed by setup-rust-vim.sh'

dry_run=false

usage() {
    cat <<EOF
Usage: ./${SCRIPT_NAME} [--dry-run]

Install or update a Vim + rust-analyzer development environment for Linux/WSL.

Options:
  --dry-run  Print the planned actions without changing the system.
  -h, --help Show this help.
EOF
}

log() {
    printf '[%s] %s\n' "$SCRIPT_NAME" "$*"
}

fail() {
    printf '[%s] error: %s\n' "$SCRIPT_NAME" "$*" >&2
    exit 1
}

run() {
    if "$dry_run"; then
        printf '[dry-run]'
        printf ' %q' "$@"
        printf '\n'
    else
        "$@"
    fi
}

require_command() {
    local command_name="$1"
    local install_hint="$2"

    command -v "$command_name" >/dev/null 2>&1 || fail "${command_name} is required. ${install_hint}"
}

install_or_update_plugin() {
    local repository="$1"
    local directory_name="$2"
    local destination="${package_root}/${directory_name}"

    if [[ -d "${destination}/.git" ]]; then
        log "Updating ${directory_name}"
        run git -C "$destination" pull --ff-only
    elif [[ -e "$destination" ]]; then
        fail "${destination} already exists but is not a Git checkout; leaving it untouched"
    else
        log "Installing ${directory_name}"
        run git clone --depth 1 "$repository" "$destination"
    fi
}

write_vim_config() {
    if [[ -f "$config_file" ]] && ! grep -Fq "$MANAGED_MARKER" "$config_file"; then
        fail "${config_file} exists and is not managed by this script; leaving it untouched"
    fi

    if "$dry_run"; then
        log "Would write managed Vim configuration to ${config_file}"
        return
    fi

    mkdir -p "$config_root"

    cat >"$config_file" <<'VIM'
" Managed by setup-rust-vim.sh
" Re-run the repository script to update this file.

syntax enable
filetype plugin indent on

set completeopt=menuone,noinsert,noselect
let g:asyncomplete_auto_popup = 1
let g:asyncomplete_auto_completeopt = 0

if executable('rust-analyzer')
    augroup rust_with_ai_lsp_setup
        autocmd!
        autocmd User lsp_setup call lsp#register_server({
                    \ 'name': 'rust-analyzer',
                    \ 'cmd': {server_info -> ['rust-analyzer']},
                    \ 'allowlist': ['rust'],
                    \ })
    augroup END
endif

function! s:on_lsp_buffer_enabled() abort
    setlocal omnifunc=lsp#complete
    setlocal signcolumn=yes
    if exists('+tagfunc')
        setlocal tagfunc=lsp#tagfunc
    endif

    nmap <buffer> gd <plug>(lsp-definition)
    nmap <buffer> gr <plug>(lsp-references)
    nmap <buffer> gi <plug>(lsp-implementation)
    nmap <buffer> gt <plug>(lsp-type-definition)
    nmap <buffer> K <plug>(lsp-hover)
    nmap <buffer> <leader>rn <plug>(lsp-rename)
    nmap <buffer> [g <plug>(lsp-previous-diagnostic)
    nmap <buffer> ]g <plug>(lsp-next-diagnostic)
endfunction

augroup rust_with_ai_lsp_buffer
    autocmd!
    autocmd User lsp_buffer_enabled call s:on_lsp_buffer_enabled()
augroup END
VIM

    log "Wrote ${config_file}"
}

while (($# > 0)); do
    case "$1" in
        --dry-run)
            dry_run=true
            ;;
        -h | --help)
            usage
            exit 0
            ;;
        *)
            usage >&2
            fail "unknown option: $1"
            ;;
    esac
    shift
done

require_command git "On Debian/Ubuntu: sudo apt install git"
require_command vim "On Debian/Ubuntu: sudo apt install vim"
require_command rustup "Install Rust with rustup first: https://rustup.rs"

if ! vim -Nu NONE -n -es \
    -c 'if !has("job") || !has("channel") || !has("lambda") | cquit 1 | endif' \
    -c 'qa!' </dev/null; then
    fail "Vim needs +job, +channel, and +lambda support; install a full Vim 8+ build"
fi

readonly package_root="${HOME}/.vim/pack/rust-with-ai/start"
readonly config_root="${HOME}/.vim/plugin"
readonly config_file="${config_root}/rust-with-ai-lsp.vim"

run mkdir -p "$package_root"

log "Installing Rust toolchain components"
run rustup component add rust-analyzer rust-src rustfmt

install_or_update_plugin "https://github.com/rust-lang/rust.vim.git" "rust.vim"
install_or_update_plugin "https://github.com/prabirshrestha/vim-lsp.git" "vim-lsp"
install_or_update_plugin "https://github.com/prabirshrestha/asyncomplete.vim.git" "asyncomplete.vim"
install_or_update_plugin "https://github.com/prabirshrestha/asyncomplete-lsp.vim.git" "asyncomplete-lsp.vim"

write_vim_config

if ! "$dry_run"; then
    command -v rust-analyzer >/dev/null 2>&1 || fail "rust-analyzer was installed but is not available on PATH"
    log "rust-analyzer: $(rust-analyzer --version)"
fi

log "Setup complete. Open a Cargo project in Vim and run :LspStatus after rust-analyzer starts."
