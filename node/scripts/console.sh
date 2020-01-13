#!/bin/bash

BOB_RPC_PORT=30334
NODE_NAME=experimental-node

BASE_PATH_BASE=/tmp

if [[ $(pwd) == *"/staging/"* ]]; then
  BASE_PATH_BASE=/tmp/staging
  mkdir $BASE_PATH_BASE
fi

case $1 in
purge)
  rm -rf $BASE_PATH_BASE/*alice*
  rm -rf $BASE_PATH_BASE/*bob*
  rm -rf $BASE_PATH_BASE/*dev*
;;
dev)
  shift
  "./target/release/${NODE_NAME}" \
      --base-path $BASE_PATH_BASE/dev \
      --dev \
      --rpc-cors=all \
      --execution=Wasm \
      --validator \
      --listen-addr=/ip4/127.0.0.1/tcp/9998 \
      --no-mdns

;;
start)
  shift
  case $1 in
  alice)
    shift
    "./target/release/${NODE_NAME}" \
        --base-path $BASE_PATH_BASE/alice \
        --chain=chain.json \
        --rpc-cors all \
        --alice \
        --node-key 0000000000000000000000000000000000000000000000000000000000000001 \
        --validator "$@"
  ;;
  bob)
    shift
    "./target/release/${NODE_NAME}" \
        --base-path $BASE_PATH_BASE/bob \
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
wrap-build)
  export PATH="/media/disk2/workspace/staging/experimental-node/node/scripts/ccwrapper:$PATH"
  echo "$(date) | wrap-build" >> /media/disk2/workspace/staging/experimental-node/node/scripts/ccwrapper/clang.log
  echo "$(date) | wrap-build" >> /media/disk2/workspace/staging/experimental-node/node/scripts/ccwrapper/ar.log
  shift
  cargo build --release "$@"
esac