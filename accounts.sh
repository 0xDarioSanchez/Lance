#!/bin/bash
#set -e

echo "        ********************************"
echo -e "\t***** Creating accounts *****..."
echo "        ********************************"
# stellar keys address lance-employee-1
stellar keys generate lance-admin --network testnet --fund
stellar keys generate lance-employee-1 --network testnet --fund
stellar keys generate lance-employee-2 --network testnet --fund
stellar keys generate lance-employer-1 --network testnet --fund
stellar keys generate lance-employer-2 --network testnet --fund
stellar keys generate lance-judge-1 --network testnet --fund
stellar keys generate lance-judge-2 --network testnet --fund
stellar keys generate lance-judge-3 --network testnet --fund

echo "        *********************************"
echo -e "\t***** Accounts generated *****..."
echo "        *********************************"