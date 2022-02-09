Feature: ParaTesting

  Scenario: spawn allychains network and check allychains
    Given a test network
    Then sleep 200 seconds
    Then launch 'node' with parameters '--unhandled-rejections=strict /usr/local/bin/simnet_scripts test_allychain ./configs/adder.json ws://localhost:11222 100 10'
