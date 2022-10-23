if [ "$#" -ne 1 ]; then
    echo "expected prod-testnet|prod-live"
    exit 1;
fi

# Based on https://andrew.red/posts/how-to-load-dotenv-file-from-shell

CONFIGURATION=$1;

(echo "$CONFIGURATION" | grep -Eq  "^prod-testnet|prod-live$") \
 && echo "Using $CONFIGURATION .." || (echo "$CONFIGURATION is not a valid (prod-testnet|prod-live ..")

echo "Removing previous settings"
(cd ../external-nginx-configuration/scripts \
 && ./delete_nginx.sh "$CONFIGURATION")

echo "Setting up a new front facing nginx"
(cd ../external-nginx-configuration/scripts \
 && ./setup_nginx.sh "$CONFIGURATION")
