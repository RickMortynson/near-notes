#!/bin/bash

mkdir out
near deploy --wasmFile ./out/near_todo_categories.wasm --accountId categories.unicorny.testnet
near deploy --wasmFile ./out/near_todo_tasks.wasm --accountId notes.unicorny.testnet
