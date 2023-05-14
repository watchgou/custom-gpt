#!/bin/bash

export CHAT_GPT_MODEL='gpt-3.5-turbo'

export MANY_CHAT=1

PROJECT_NAME='custom-gpt'
case "$1" in

start)
        CURRENT_DIR=$(cd `dirname $0`; pwd)
        nohup $CURRENT_DIR/target/release/$PROJECT_NAME >/dev/null 2>&1 &

        ;;
stop)
        PID=$(ps -ef | grep $PROJECT_NAME | grep -v grep | awk 'NR==1 {print $2}')
        if [ $PID ]; then
                kill -9 $PID
                echo "Kill process pid: ${PID}"
        fi
        ;;
build-start)
        cargo clean
        cargo build --release
        "$0" start
        ;;

esac
