 if [ "$#" -ne 1 ]; then
     echo "expected test-testnet|accept-testnet|accept-live|prod-testnet|prod-live"
     exit 1;
 fi

CONFIGURATION=$1

(echo "$CONFIGURATION" | grep -Eq  "^test-testnet|accept-testnet|accept-live|prod-testnet|prod-live$" && echo "Using $CONFIGURATION ..") \
 || { echo "$CONFIGURATION is not a valid (test-testnet|accept-testnet|accept-live|prod-testnet|prod-live) configuration .."; exit 1; }

set -a; source ../configs/"$CONFIGURATION"/.env; set +a

 (cd ../../../../backend/backend && cargo build) && \
 rust-gdb --args ../../../../backend/backend/target/debug/btc-chat-backend \
 "@../configs/${CONFIGURATION}/config/general.conf" \
 "@../configs/${CONFIGURATION}/config/btc_pay.conf" \
 "@../configs/${CONFIGURATION}/config/database.conf" \
 "@../configs/${CONFIGURATION}/config/invoice.conf" \
 "@../configs/${CONFIGURATION}/config/keycloak.conf" \
 "@../configs/${CONFIGURATION}/config/web_socket.conf" \
 "@../configs/${CONFIGURATION}/config/server.conf" \
 "@../configs/${CONFIGURATION}/config/test_webhook.conf"
