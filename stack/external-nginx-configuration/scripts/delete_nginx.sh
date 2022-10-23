if [ "$#" -ne 1 ]; then
    echo "expected prod-testnet|prod-live"
    exit 1;
fi

exit 1 # script disabled for now

CONFIGURATION=$1;

(echo "$CONFIGURATION" | grep -Eq  "^prod-testnet|prod-live$") \
 && echo "Using $CONFIGURATION .." || (echo "$CONFIGURATION is not a valid (prod-testnet|prod-live ..")

set -a; source ../configs/"$CONFIGURATION"/.env; set +a

[ -z "$DOMAIN" ] && { echo "DOMAIN must be set otherwise we do great damage"; exit 1; }


echo "Purging nginx";
sudo apt purge -y nginx;

echo "Cleaning up /var/www/$DOMAIN";
rm -r /var/www/"$DOMAIN";

echo "Cleaning up /etc/nginx/sites-available/$DOMAIN";
rm -r /etc/nginx/sites-available/"$DOMAIN"

echo "Cleaning up /etc/nginx/sites-enabled/$DOMAIN";
rm /etc/nginx/sites-enabled/"$DOMAIN"


