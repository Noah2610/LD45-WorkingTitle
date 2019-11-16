# Shared bash code for travis scripts.

# shellcheck source=./util.sh source=./share.sh
_dir="$( dirname "$0" )"
[ -f "${_dir}/util.sh" ] || bash "${_dir}/download-util.sh" || exit 1
source "${_dir}/util.sh"
unset _dir

# Only run if this script was started from Travis
[ -z "$TRAVIS" ] && err "This script should only be run from Travis"

function get_release_path {
    check "cat"
    local file_with_path="${ROOT}/.travis-release-path-${TARGET}"
    check_file "$file_with_path"
    cat "$file_with_path"
}

function pushd_wrapper {
    \pushd "$@" &> /dev/null || exit 1
}
function popd_wrapper {
    \popd "$@" &> /dev/null || exit 1
}

# Copied from util.sh
# Tries to run the given command and hides its output.
# If the command fails, then it prints the output with `err`.
# If the variable `$also_to_stderr` is set, then additionally to writing
# the commands output to the `$LOGFILE`, it also prints it to stderr.
function try_run {
    local cmd="$1"
    [ -z "$cmd" ] && err "No command given."
    local out
    local out_files=("$LOGFILE")
    msg "${spacing}Running: \033[${COLOR_CODE}m${cmd}\033[m"
    if ! $cmd 2>&1 | tee -a "${out_files[@]}"; then
        err "Command failed:\n  \033[${COLOR_CODE}m${cmd}\033[m\nReturned:\n${out}"
    fi
}

function send_telegram_message {
    { [ -z "$TELEGRAM_TOKEN" ] || [ -z "$TELEGRAM_CHAT_ID" ];  } \
        && return 0

    local message="$1"
    [ -z "$message" ] && err "Function \`send_telegram_message\` requires one argument as the message to send"

    local url="https://api.telegram.org/bot${TELEGRAM_TOKEN}/sendMessage"

    curl -s -X POST "$url" \
        -d chat_id="$TELEGRAM_CHAT_ID" \
        -d text="$message" \
        -d parse_mode="Markdown" &> /dev/null
}

alias pushd="pushd_wrapper"
alias popd="popd_wrapper"

_logdir="${ROOT}/logs"
[ -d "$_logdir" ] || mkdir -p "$_logdir"
LOGFILE="${_logdir}/$( basename "$0" ).log"
unset _logdir

TARGET="$TRAVIS_OS_NAME"
RELEASE_TARGETS="$TARGET"
EXE_NAME="$RELEASE_EXE_NAME_OUTPUT"
