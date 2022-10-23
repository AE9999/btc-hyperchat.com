# Installing NGINX. Inspired by https://www.digitalocean.com/community/tutorials/how-to-install-nginx-on-ubuntu-18-04

if [ "$#" -ne 1 ]; then
    echo "expected prod-testnet|prod-live"
    exit 1;
fi

CONFIGURATION=$1;

(echo "$CONFIGURATION" | grep -Eq  "^prod-testnet|prod-live$") \
 && echo "Using $CONFIGURATION .." || (echo "$CONFIGURATION is not a valid (prod-testnet|prod-live ..")

set -a; source ../configs/"$CONFIGURATION"/.env; set +a

which envsubst || { echo "Please install envsubst .."; exit 1; }
[ -z "$DOMAIN" ] && { echo "DOMAIN must be set otherwise we do great damage"; exit 1; }

echo "Installing nginx .."
sudo apt update && sudo apt install -y nginx

echo "Setting up letsencrypt for $DOMAIN .."
sudo add-apt-repository ppa:certbot/certbot
sudo apt-get update
sudo apt install python3-certbot-nginx

#
# The below command will cause an exception due to the fact that we are using Ubuntu 20.04.
# I used the workaround described here https://community.letsencrypt.org/t/ubuntu-20-04-any-tips-attributeerror-module-acme-challenges-has-no-attribute-tlssni01/115831.
#
sudo certbot --nginx -d ${DOMAIN}

echo "Disabeling nginx to put in the the correct final config"
sudo systemctl stop nginx

sudo cat ../templates/conf.template  | envsubst '$DOMAIN' > "/etc/nginx/sites-available/default";

echo "Testing nginx -t "
sudo nginx -t || { echo "NGINX sanity test failed"; exit 1; }

echo "Done. Restarting nginx server .."
sudo systemctl start nginx
