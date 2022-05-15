#! /bin/bash

while :
do
echo "1. Start Python Websocket Server"
echo "2. Start Node HTTP Server"
echo "3. Build and run Rust Keylogger Client"
echo "4. Quit"
read -n 1 -t 15 a
printf "\n"
case $a in
1* )    echo "Starting Python WS Server";
        gnome-terminal -- python3 python_ws_server/server.py ;;
 
2* )    echo "Starting Node HTTP Server";
        gnome-terminal -- node express_http_server/server.js ;;
 
3* )    echo "Building and running Rust Keylogger Client";
        gnome-terminal -- cargo build && sudo ./target/debug/rustkeyinput ;;

4* )     exit 0;;

* )     echo "Try again.";;
esac
done