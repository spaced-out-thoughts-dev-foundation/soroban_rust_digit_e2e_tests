# frozen_string_literal: true

require 'spec_helper'

RSpec.describe 'Contract Translation Rust --> DTR --> Rust' do
  let(:base_directory) { './stellar_soroban' }
  let(:official_directory) { "#{base_directory}/official_sdf_examples" }
  let(:unofficial_directory) { "#{base_directory}/unofficial_digicus_examples" }

  context 'when official SDF Example' do
    it 'translates the account contract' do
      assert_translates_rust_to_dtr_and_back("#{official_directory}/account/", x: '1', i: 1, t: 22)
    end

    it 'translates the alloc contract' do
      assert_translates_rust_to_dtr_and_back("#{official_directory}/alloc/", x: '1', i: 2, t: 22)
    end

    it 'translates the atomic_multiswap contract' do
      assert_translates_rust_to_dtr_and_back("#{official_directory}/atomic_multiswap/", x: '1', i: 3, t: 22)
    end

    it 'translates the atomic_swap contract' do
      assert_translates_rust_to_dtr_and_back("#{official_directory}/atomic_swap/", x: '1', i: 4, t: 22)
    end

    it 'translates the auth contract' do
      assert_translates_rust_to_dtr_and_back("#{official_directory}/auth/", x: '11', i: 5, t: 22)
    end

    it 'translates the cross contract contracts' do
      assert_translates_rust_to_dtr_and_back_multi_contract_directory("#{official_directory}/cross_contract/contract_b", [
                                                                        "#{official_directory}/cross_contract/contract_a"
                                                                      ], x: '11', i: 6, t: 22)
    end

    it 'translates the custom_types contract' do
      assert_translates_rust_to_dtr_and_back("#{official_directory}/custom_types/", x: '11', i: 7, t: 22)
    end

    it 'translates the deployer contract' do
      assert_translates_rust_to_dtr_and_back_multi_contract_directory("#{official_directory}/deployer/deployer", [
                                                                        "#{official_directory}/deployer/contract"
                                                                      ], x: '1', i: 8, t: 22)
    end

    it 'translates the errors contract' do
      assert_translates_rust_to_dtr_and_back("#{official_directory}/errors/", x: '11', i: 9, t: 22)
    end

    it 'translates the eth_abi contract' do
      assert_translates_rust_to_dtr_and_back("#{official_directory}/eth_abi/", x: '10', i: 10, t: 22)
    end

    it 'translates the events contract' do
      assert_translates_rust_to_dtr_and_back("#{official_directory}/events/", x: '11', i: 11, t: 22)
    end

    it 'translates the fuzzing contract' do
      assert_translates_rust_to_dtr_and_back("#{official_directory}/fuzzing/", x: '1', i: 12, t: 22)
    end

    it 'translates the hello world contract' do
      assert_translates_rust_to_dtr_and_back("#{official_directory}/hello_world/", x: '11', i: 13, t: 22)
    end

    it 'translates the increment contract' do
      assert_translates_rust_to_dtr_and_back("#{official_directory}/increment/", x: '11', i: 14, t: 22)
    end

    it 'translates the logging contract' do
      assert_translates_rust_to_dtr_and_back("#{official_directory}/logging/", x: '11', i: 15, t: 22)
    end

    it 'translates the mint_lock contract' do
      assert_translates_rust_to_dtr_and_back("#{official_directory}/mint_lock/", x: '1', i: 16, t: 22)
    end

    it 'translates the simple_account contract' do
      assert_translates_rust_to_dtr_and_back("#{official_directory}/simple_account/", i: 17, t: 22)
    end

    it 'translates the single_offer contract' do
      assert_translates_rust_to_dtr_and_back("#{official_directory}/single_offer/", i: 18, t: 22)
    end

    it 'translates the timelock contract' do
      assert_translates_rust_to_dtr_and_back("#{official_directory}/timelock/", x: '1', i: 19, t: 22)
    end

    it 'translates the ttl contract' do
      assert_translates_rust_to_dtr_and_back("#{official_directory}/ttl/", x: '11', i: 20, t: 22)
    end

    it 'translates the upgradable contract old contract' do
      assert_translates_rust_to_dtr_and_back_multi_contract_directory("#{official_directory}/upgradable_contract/old_contract", [
                                                                        "#{official_directory}/upgradable_contract/new_contract"
                                                                      ], x: '11', i: 21, t: 22)
    end

    it 'translates the workspace contracts' do
      assert_translates_rust_to_dtr_and_back_multi_contract_directory("#{official_directory}/workspace/contract_b", [
                                                                        "#{official_directory}/workspace/contract_a_interface", "#{official_directory}/workspace/contract_a"
                                                                      ], x: '1', i: 22, t: 22)
    end
  end
end
