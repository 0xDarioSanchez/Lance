#!/bin/bash
#set -e

echo "***************************"
echo -e "\t*****Building*****..."
echo "***************************"
cargo build --target wasm32v1-none --release && stellar contract optimize --wasm target/wasm32v1-none/release/lance.wasm

echo "*************************"
echo -e "\t****Deploying**** ..."
echo "*************************"
stellar contract deploy \
    --wasm target/wasm32v1-none/release/practice.optimized.wasm \
    --alias contract_address \
    --source lancer-admin \
    --network testnet \
    -- \
    --admin lancer-admin \
    --token CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC 

echo "**************************************"
echo -e "\tRegistration of Employee 1 ..."
echo "**************************************"
stellar contract invoke \
    --id contract_address \
    --source lancer-employee-1 \
    --network testnet \
    -- nuevo_usuario \
    --user lancer-employee-1 \
    --is_employer false \
    --is_employee true \
    --is_judge false \
    --personal_data "Employee 1"

echo "*******************************"
echo -e "\tGet Employee 1 ..."
echo "********************************"
stellar contract invoke \
    --id contract_address \
    --source lancer-employee-1 \
    --network testnet \
    -- get_user \
    --user lancer-employee-1

echo "**************************************"
echo -e "\tRegistration Employer 1 ..."
echo "**************************************"
stellar contract invoke \
    --id contract_address \
    --source admin \
    --network testnet \
    -- create_ong \
    --ong GDVC5X7T4HKW42TAF7CTYH4I4WS4UJPNDVGOLCH2LWOIAMZ5C5V6JIEI

echo "*****************************************"
echo -e "\tCreation Service 1 from Employer 1 ..."
echo "*****************************************"
stellar contract invoke \
    --id contract_address \
    --source alice-ong-1 \
    --network testnet \
    -- create_campaign \
    --creator GAB3GGZD77VPZDXKF33SDPETZEBCY3ATPAPQBCDK42MGDDFHCHSTT7EZ \
    --beneficiary GBYO36NUZL4RE4INJ6726IETX7FR5BJTX6PWJL7XWWRJOZD2XNMHGPQV \
    --goal 100000000 \
    --min_donation 100000

echo "*****************************************"
echo -e "\tAccepting Service 1 from Employer 1 ..."
echo "*****************************************"
stellar contract invoke \
    --id contract_address \
    --source alice-ong-2 \
    --network testnet \
    -- create_campaign \
    --creator GDVC5X7T4HKW42TAF7CTYH4I4WS4UJPNDVGOLCH2LWOIAMZ5C5V6JIEI \
    --beneficiary GBQDY66S67DAGZMGIWXHDK2AYKAAQD7HJV3H6HAB4T4KPUWBMKXT65Y4 \
    --goal 100000000 \
    --min_donation 100000


echo "**********************************************************"
echo -e "\tApproving milestione for Service 1 from Employer 1 ..."
echo "**********************************************************"
stellar contract invoke \
    --id contract_address \
    --source alice-ong-2 \
    --network testnet \
    -- create_campaign \
    --creator GDVC5X7T4HKW42TAF7CTYH4I4WS4UJPNDVGOLCH2LWOIAMZ5C5V6JIEI \
    --beneficiary GC3GWMVITNHCT6LS4URAGGUUUMH47BVMZ5VMSRZPCG6GVDT66M23JAEJ \
    --goal 100000000 \
    --min_donation 100000



# echo "******************************************************"
# echo -e "\tOpening contract on Steller Expert explorer"
# echo "******************************************************"

# CONTRACT_ID=$(stellar contract alias show contract_address)
# EXPLORER_URL="https://stellar.expert/explorer/testnet/contract/$CONTRACT_ID"
# xdg-open "$EXPLORER_URL"