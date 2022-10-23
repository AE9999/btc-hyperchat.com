if [ "$#" -ne 1 ]; then
    echo "expected test-testnet|accept-testnet|accept-live|prod-testnet|prod-live"
    exit 1;
fi

# Based on https://andrew.red/posts/how-to-load-dotenv-file-from-shell

CONFIGURATION=$1;

(echo "$CONFIGURATION" | grep -Eq  "^test-testnet|accept-testnet|accept-live|prod-testnet|prod-live$" && echo "Using $CONFIGURATION ..") \
 || { echo "$CONFIGURATION is not a valid (test-testnet|accept-testnet|accept-live|prod-testnet|prod-live) configuration .."; exit 1; }

set -a; source ../configs/"$CONFIGURATION"/.env; set +a


# TODO MOVE THIS TO A BUILD CONTAINER.
which diesel || { echo "Please install Diesel with the postgres features enabled (https://diesel.rs/guides/getting-started).."; exit 1; }



PSQL="PGPASSWORD=$POSTGRES_PASSWORD psql -U $POSTGRES_USER -h localhost ";
DOCKER_COMMAND="docker exec $POSTGRES_CONTAINER_NAME"
CREATE_BTC_DB_COMMAND="$DOCKER_COMMAND sh -c '$PSQL -c \"create database $POSTGRES_BTC_CHAT_DB;\"'"
CREATE_KEYCLOAK_DB_COMMAND="$DOCKER_COMMAND sh -c '$PSQL -c \"create database $POSTGRES_KEYCLOAK_DB;\"'"

testDbConnection() {
  #
  # # which psql || { echo "Please install psql client (sudo apt-get install -y postgresql-client) .."; exit 1; }
  # Nicer way but requires users to have psql client installed.
  # "PGPASSWORD=$POSTGRES_PASSWORD" psql -U "$POSTGRES_USER" -h localhost -c 'SELECT datname FROM pg_database;';
  # echo $?
  #
  #
  return 0
}

waitForStartup () {
  sleep 10; # Kan netter
  until testDbConnection; do sleep 5; echo "waiting for server to come up"; done;
}

(docker volume create "$POSTGRES_VOLUME" \
 && docker-compose up -d \
 && waitForStartup \
 && echo "Going to execute $CREATE_BTC_DB_COMMAND .." \
 && sh -c "$CREATE_BTC_DB_COMMAND" \
 && echo "Going to execute $CREATE_KEYCLOAK_DB_COMMAND .." \
 && sh -c "$CREATE_KEYCLOAK_DB_COMMAND" \
 && echo "Executing the diesel migration .." \
 && (cd ../../../../backend/backend \
     && diesel migration run --database-url="postgres://$POSTGRES_USER:$POSTGRES_PASSWORD@localhost:5432/$POSTGRES_BTC_CHAT_DB") \
 && docker-compose down )


