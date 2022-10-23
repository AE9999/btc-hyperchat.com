if [ "$#" -ne 1 ]; then
    echo "expected test-testnet|accept-testnet|accept-live|prod-testnet|prod-live"
    exit 1;
fi

# Based on https://andrew.red/posts/how-to-load-dotenv-file-from-shell

CONFIGURATION=$1;

(echo "$CONFIGURATION" | grep -Eq  "^test-testnet|accept-testnet|accept-live|prod-testnet|prod-live$" && echo "Using $CONFIGURATION ..") \
 || { echo "$CONFIGURATION is not a valid (test-testnet|accept-testnet|accept-live|prod-testnet|prod-live) configuration .."; exit 1; }

set -a; source ../configs/"$CONFIGURATION"/.env; set +a

echo "Checking all required programs .."

which npm || { echo "Please install NPM .."; exit 1; }
which jq || { echo "Please install JQ .."; exit 1; }
which docker || { echo "Please install Docker .."; exit 1; }
which docker-compose || { echo "Please install Docker Compose .."; exit 1; }

echo "Installing the internal jwt converter "
(cd ../jwt-converter/ \
 && npm install \
 && npm link) || { echo "Could not install JWT converter .."; exit 1; }

which btc-chat-convert-jwt-token || { echo "Converter unavailable .."; exit 1; }

CLI="cd /opt/keycloak/bin/ && ./kcadm.sh"

AUTHENTICATE_CMD="$CLI config \
                       credentials \
                       --server http://localhost:8080 \
                       --realm master \
                       --user ${KEYCLOAK_USER} \
                       --password ${KEYCLOAK_PASSWORD}";

HEADER="\"frame-src 'self'; frame-ancestors 'self' ${KEYCLOAK_FRONTEND_URL}; object-src 'none';\""

CREATE_REALM_CMD="$CLI create \
                       realms \
                       -s realm=${KEYCLOAK_BTC_CHAT_REALM_NAME} \
                       -s enabled=true \
                       -s eventsEnabled=true \
                       -s registrationAllowed=${KEYCLOAK_BTC_CHAT_ALLOW_USER_REGISTRATION} \
                       -s eventsListeners=[\\\"jboss-logging\\\",\\\"registration-event-listener\\\"] \
                       -s browserSecurityHeaders.contentSecurityPolicy=$HEADER \
                       -o";

CREATE_CLIENT_CMD="$CLI create \
                        clients \
                        -r ${KEYCLOAK_BTC_CHAT_REALM_NAME} \
                        -s clientId=${KEYCLOAK_BTC_CHAT_CLIENT_NAME} \
                        -s enabled=true \
                        -s publicClient=true \
                        -s redirectUris=[\\\"${KEYCLOAK_FRONTEND_URL}/*\\\"] \
                        -s webOrigins=[\\\"${KEYCLOAK_FRONTEND_URL}\\\"] \
                        -s attributes.\\\"post.logout.redirect.uris\\\"=\"${KEYCLOAK_FRONTEND_URL}\" \
                        -o ";

DOCKER_COMMAND="docker exec $KEYCLOAK_CONTAINER_NAME"
AUTHENTICATE_CMD="$DOCKER_COMMAND sh -c '$AUTHENTICATE_CMD'"
CREATE_REALM_CMD="$DOCKER_COMMAND sh -c '$CREATE_REALM_CMD'"
CREATE_CLIENT_CMD="$DOCKER_COMMAND sh -c '$CREATE_CLIENT_CMD'"

waitForStartup () {
   sh -c 'until [ "$(curl -s -o /dev/null -w ''%{http_code}'' localhost:8080)" -eq 200 ]; do sleep 5; echo "waiting for server to come up"; done'
}

(cd .. \
 && echo "Starting up docker configuration " \
 && docker-compose up -d \
 && waitForStartup \
 && echo "Server started up! " \
 && echo "Going to execute $AUTHENTICATE_CMD .." \
 && sh -c "$AUTHENTICATE_CMD" \
 && echo "Going to execute $CREATE_REALM_CMD .." \
 && sh -c "$CREATE_REALM_CMD" \
 && echo "Going to execute $CREATE_CLIENT_CMD .." \
 && sh -c "$CREATE_CLIENT_CMD" \
 && {
  TO_DECODE=$(curl http://localhost:8080/realms/${KEYCLOAK_BTC_CHAT_REALM_NAME}/protocol/openid-connect/certs | jq -c '.keys | .[] | select( .alg == "RS256" )') && \
  echo "Setup your app with key $TO_DECODE, you can convert it at https://8gwifi.org/jwkconvertfunctions.jsp, but we will try to do so automatically." \
  && echo "Writing generated to backend-config at $BTC_CHAT_BACKEND_CERTIFICATE_LOCATION .." \
  && (echo "$TO_DECODE" | btc-chat-convert-jwt-token > "$DB_AND_KEYCLOAK_ONLY_BTC_CHAT_BACKEND_CERTIFICATE_LOCATION") \
  && echo "Copying the certificate for use in the internal nginx configuration which will use this db .." \
  && (cp "$DB_AND_KEYCLOAK_ONLY_BTC_CHAT_BACKEND_CERTIFICATE_LOCATION" "$INTERNAL_NGINX_BACKEND_BTC_CHAT_BACKEND_CERTIFICATE_LOCATION")
 } \
 &&  docker-compose down \
 && echo "Configuration is done ..")

# dump certificates





