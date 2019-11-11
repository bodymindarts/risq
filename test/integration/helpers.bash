#!/usr/bin/env bats

# Point to our local risq binary!
if [ -z "${RISQ_BIN_DIR}" ]; then
    echo "Must set RISQ_BIN_DIR variable to a location that contains risq binary!"
    exit 1
fi

risq=${RISQ_BIN_DIR}/risq

test_tmp_dir() {
  mkdir -p ${BATS_TMPDIR}/${BATS_TEST_NAME}
  echo ${BATS_TMPDIR}/${BATS_TEST_NAME}
}

start_node() {
  echo ${2} > $(test_tmp_dir)/${1}_port
  echo ${3} > $(test_tmp_dir)/${1}_api_port
  background "${risq}" d --api-port ${3} -p ${2} -n BtcRegtest \
          --force-seed $(seed_addr) --no-tor > $(test_tmp_dir)/${1}_pid
}

node_status() {
  api_port=$(cat $(test_tmp_dir)/${1}_api_port)
  curl -s "127.0.0.1:${api_port}/status" | jq -r "${2}"
}

start_dummy_seed() {
  echo ${1} > $(test_tmp_dir)/dummy_seed_port
  background "${risq}" dummy-seed -p ${1} > $(test_tmp_dir)/dummy_seed_pid
}

seed_addr() {
  echo "127.0.0.1:$(cat $(test_tmp_dir)/dummy_seed_port)"
}

stop_node() {
  kill $(cat $(test_tmp_dir)/${1}_pid) > /dev/null
}

stop_dummy_seed() {
  kill $(cat $(test_tmp_dir)/dummy_seed_pid) > /dev/null
}

# Run the given command in the background. Useful for starting a
# node and then moving on with commands that exercise it for the
# test.
#
# Ensures that BATS' handling of file handles is taken into account;
# see
# https://github.com/bats-core/bats-core#printing-to-the-terminal
# https://github.com/sstephenson/bats/issues/80#issuecomment-174101686
# for details.
background() {
  "$@" 3>- &
  echo $!
}

# Stolen from
# https://github.com/docker/swarm/blob/master/test/integration/helpers.bash
retry() {
  local attempts=$1
  shift
  local delay=$1
  shift
  local i

  for ((i=0; i < attempts; i++)); do
    run "$@"
    # shellcheck disable=2154
    if [[ "$status" -eq 0 ]] ; then
      return 0
    fi
    sleep "$delay"
  done

    # shellcheck disable=2154
    echo "Command \"$*\" failed $attempts times. Output: $output"
    false
  }
