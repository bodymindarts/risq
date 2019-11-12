#!/usr/bin/env bats

load "helpers"

setup() {
  start_dummy_seed $(expr 10000 + ${BATS_TEST_NUMBER})
}

teardown() {
  stop_dummy_seed
}

@test "alice connects to seed" {
  start_node "alice" 1101 1111

  retry 5 1 [ "$(node_status alice '.connections | length')" == "1" ]
  [ "$(node_status alice '.connections | to_entries | .[0].value.addr')" == "127.0.0.1:10001" ]

  stop_node "alice"
}

@test "alice and bob connect" {
  start_node "alice" 2101 2111
  start_node "bob" 2201 2211


  retry 5 1 [ "$(node_status alice '.connections | length')" == "2" ]

  jq_expr=".connections | to_entries | [ .[].value.addr ] | sort == [ \"$(seed_addr)\", \"$(node_addr bob)\" ] "
  [ "$(node_status alice "${jq_expr}" )" = "true" ]

  stop_node "bob"
  stop_node "alice"
}
