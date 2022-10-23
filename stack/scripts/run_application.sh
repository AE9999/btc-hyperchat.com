if [ "$#" -ne 1 ]; then
    echo "expected prod-testnet|prod-live"
    exit 1;
fi

# Based on https://andrew.red/posts/how-to-load-dotenv-file-from-shell

CONFIGURATION=$1;

(echo "$CONFIGURATION" | grep -Eq  "^prod-testnet|prod-live$") \
 && echo "Using $CONFIGURATION .." || (echo "$CONFIGURATION is not a valid (prod-testnet|prod-live ..")

(cd ./create && ./create_application_docker_images.sh "$CONFIGURATION");

(cd ../application-configuration/internal-nginx-configuration/scripts \
 && ./run.sh "$CONFIGURATION")
