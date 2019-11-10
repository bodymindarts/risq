#!/usr/bin/env bats

load "helpers"

setup() {
  start_dummy_seed 1100
}

teardown() {
  stop_dummy_seed
}

@test "alice connects to seed" {
  start_node "alice" 1101 1111

  [ "$(node_status alice '.connections | length')" = "1" ]
  [ "$(node_status alice '.connections | to_entries | .[0].value.addr')" == "127.0.0.1:1100" ]

  stop_node "alice"
}
