#!/bin/bash

BOB_RPC_PORT=30334
NODE_NAME=experimental-node

case $1 in
purge)
  rm -rf /tmp/*alice*
  rm -rf /tmp/*bob*
  rm -rf /tmp/*dev*
;;
dev)
  shift
  "./target/release/${NODE_NAME}" \
      --base-path /tmp/dev \
      --chain=dev \
      --rpc-cors=all
;;
start)
  shift
  case $1 in
  alice)
    shift
    "./target/release/${NODE_NAME}" \
        --base-path /tmp/alice \
        --chain=chain.json \
        --rpc-cors all \
        --alice \
        --node-key 0000000000000000000000000000000000000000000000000000000000000001 \
        --validator "$@"
  ;;
  bob)
    shift
    "./target/release/${NODE_NAME}" \
        --base-path /tmp/bob \
        --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/QmY8R46KqvBHLwBBu8dE3LygvQdFa6usrfyVH6zLzzSjsg \
        --chain=chain.json \
        --rpc-cors all \
        --bob \
        --port "$BOB_RPC_PORT" \
        --validator "$@"
  ;;
  *)
    echo "Can't start node '$1'"
    exit -1
  esac
;;
llvm-env)
  mkdir /tmp/wasmout
  cp target/release/wbuild/target/wasm32-unknown-unknown/release/experimental_node_runtime.wasm /tmp/wasmout/final.wasm
  sudo docker run --user 0:0 \
    --volume /media/disk2/workspace/experimental-node/ring/wasmcore:/c:ro \
    --volume /media/disk2/workspace/experimental-node/ring/wasmcore/libwasmcore:/c/libwasmcore \
    --volume /tmp/wasmout/final.wasm:/tmp/final.wasm \
    --workdir /c \
    --interactive --tty --rm wehlutyk/wasm-compiler:0.3.0 \
    bash
;;
check-nm)
  llvm-nm-6.0 -a target/release/wbuild/target/wasm32-unknown-unknown/release/experimental_node_runtime.wasm
;;
esac