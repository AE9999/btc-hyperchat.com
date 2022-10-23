if [ "$#" -ne 1 ]; then
    echo "expected test-testnet|accept-testnet|accept-live|prod-testnet|prod-live"
    exit 1;
fi

# Based on https://andrew.red/posts/how-to-load-dotenv-file-from-shell

CONFIGURATION=$1;

(echo "$CONFIGURATION" | grep -Eq  "^test-testnet|accept-testnet|accept-live|prod-testnet|prod-live$" && echo "Using $CONFIGURATION ..") \
 || { echo "$CONFIGURATION is not a valid (test-testnet|accept-testnet|accept-live|prod-testnet|prod-live) configuration .."; exit 1; }


export FRONTEND_BUILD_CONFIGURATION=$CONFIGURATION;

(cd ../../../ \
 && ./gradlew keycloak:registration-event-listener:build \
 && ./gradlew backend:buildDocker \
 && ./gradlew frontend:buildDocker)
