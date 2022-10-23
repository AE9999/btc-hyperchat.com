if [ "$#" -ne 1 ]; then
    echo "expected test-testnet|accept-testnet|accept-live|prod-testnet|prod-live"
    exit 1;
fi

# Based on https://andrew.red/posts/how-to-load-dotenv-file-from-shell

CONFIGURATION=$1;

(echo "$CONFIGURATION" | grep -Eq  "^test-testnet|accept-testnet|accept-live|prod-testnet|prod-live$") \
 && echo "Using $CONFIGURATION .." || (echo "$CONFIGURATION is not a valid (test-testnet|accept-testnet|accept-live|prod-testnet|prod-live) configuration ..")

# clean up

(cd ../../application-configuration/db-only \
 && docker-compose down);

(cd ../../application-configuration/db-and-keycloak-only \
 && docker-compose down);

(cd ../../application-configuration/db-only; \
 set -a; \
 source ./configs/"$CONFIGURATION"/.env; \
 set +a \
 && docker volume rm "$POSTGRES_VOLUME")
