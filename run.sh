#!/bin/bash
#set -e

echo "************************************"
echo -e "\t*****Building*****..."
echo "************************************"
cargo build --target wasm32v1-none --release && stellar contract optimize --wasm target/wasm32v1-none/release/lance.wasm

echo "**********************************"
echo -e "\t****Deploying**** ..."
echo "**********************************"
stellar contract deploy \
  --wasm target/wasm32v1-none/release/lance.optimized.wasm \
  --source-account lance-admin \
  --network testnet \
  --alias lance \
  -- \
  --admin lance-admin \
  --token CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC \
  --blend-pool CD6SJIRPOSP5NC73ZWJ36WT3PS34M6YUMOVDALMAR74IKXWGMX5VMEAG
# CD6SJIRPOSP5NC73ZWJ36WT3PS34M6YUMOVDALMAR74IKXWGMX5VMEAG

echo "***********************************************"
echo -e "\tRegistration of Employee 1 ..."
echo "***********************************************"
stellar contract invoke \
    --id lance \
    --source lance-employee-1 \
    --network testnet \
    -- new_user \
    --user lance-employee-1 \
    --is_employer false \
    --is_employee true \
    --is_judge false \
    --personal_data "Employee 1"

echo "****************************************"
echo -e "\tGet Employee 1 ..."
echo "*****************************************"
stellar contract invoke \
    --id lance \
    --source lance-employee-1 \
    --network testnet \
    -- get_user \
    --user lance-employee-1

echo "**************************************"
echo -e "\tRegistration Employer 1 ..."
echo "**************************************"
stellar contract invoke \
    --id lance \
    --source lance-employer-1 \
    --network testnet \
    -- new_user \
    --user lance-employee-1 \
    --is_employer true \
    --is_employee false \
    --is_judge false \
    --personal_data "Employer 1"

echo "**************************************************"
echo -e "\tCreation Service 1 from Employer 1 ..."
echo "**************************************************"
stellar contract invoke \
    --id lance \
    --source lance-employee-1 \
    --network testnet \
    -- create_service \
    --creator lance-employee-1 \
    --employer lance-employer-1 \
    --id 1 \
    --duration 1 \
    --metadata null \
    --milestone_payment 1000000
        
echo "**************************************************"
echo -e "\tAccepting Service 1 from Employer 1 ..."
echo "**************************************************"
stellar contract invoke \
    --id lance \
    --source lance-employer-1 \
    --network testnet \
    -- accept_service \
    --employer lance-employer-1 \
    --id 1

echo "*******************************************************************"
echo -e "\tApproving milestione for Service 1 from Employer 1 ..."
echo "*******************************************************************"
stellar contract invoke \
    --id lance \
    --source lance-employer-1 \
    --network testnet \
    -- approve_service \
    --employer lance-employer-1 \
    --id 1


echo "**********************************************************"
echo -e "\tRediming earnings from Employee 1 ..."
echo "**********************************************************"
stellar contract invoke \
    --id lance \
    --source lance-employee-1 \
    --network testnet \
    -- redeem \
    --employee lance-employee-1


# echo "******************************************************"
# echo -e "\tOpening contract on Steller Expert explorer"
# echo "******************************************************"

CONTRACT_ID=$(stellar contract alias show lance)
EXPLORER_URL="https://stellar.expert/explorer/testnet/contract/$CONTRACT_ID"
xdg-open "$EXPLORER_URL"