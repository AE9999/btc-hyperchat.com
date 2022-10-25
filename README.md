# About

<a href="BTC-HyperChat.com">BTC-HyperChat.com</a> is an open source, Bitcoin based, Web application which allows users to get payed (in  Bitcoin) to receive and (possibly automatically) handle messages. It was developed to integrate with the <a href='https://streamlabs.com/'>streamlabs</a> api but any one using software that allows for 'webhooks' or 'callbacks' can integrate it.

# Happy Flow
The basic workflow of our app is
1. A donor registrates a message with a donation amout with our system.
2. We generate an invoice for the user to pay.
3. The donor pays the invoice.
4. We inform the user that a new donation has been made. This information is given in app, with a push message. Optionally this information is pushed to a webhook setup by the user.

# Architecture
Our application consists of the following components:
1. A <a href="https://www.postgresql.org/">PostgreSQL</a> database to store data.
2. A <a href=https://btcpayserver.org/>BtcPay</a> server to track payments. This can be a self hosted or a public instance.
3. Our backend. A <a href="https://www.rust-lang.org/">Rust</a> based API.
4. Our frontend. An <a href="https://angular.io/">Angular</a> app as a the user interface
5. A <a href="https://www.keycloak.org/">Keycloak</a> instance to do user authentication & authorization.
6. All components are deployed together by using <a href="https://www.docker.com/">Docker</a>.
7. The application is made accessible through a <a href="https://nginx.com">NGINX</a> server, which has a <a href="https://letsencrypt.org/">Let's Encrypt</a> certificate for secure communication.

# Running your own instance

If you wish to run your own instance we provide a number of scripts to help you setup your instance. For the below instructions we are assuming your are working on some sort of Ubuntu Linux distribution.

For convenience, we support a number of different environments. These are: 'test-testnet', 'accept-testnet', 'accept-live', 'prod-testnet', 'prod-live'

All scripts mentioned below should be run only in the directory they are located in.  All scripts expect their target environment to be given as argument. I.e. `./run_application.sh prod-live`

Before running these scripts you will need to configure the environment it is going to run on. This is done by editing the .env files so that their values match your environment. (Basically setting your own passwords & server locations.).

You will also need to add some configuration to the files at [frontend/angular/src/indexes](frontend/angular/src/indexes) and [frontend/angular/src/environments](frontend/angular/src/environments).

For all 'testnet' environments we use the publicly available Testnet BtcPay server https://testnet.demo.btcpayserver.org.  For live environments you will need to setup your own publicly available BtcPay server or use a publicly available <a href="https://directory.btcpayserver.org/filter/hosts">one</a>.

## Install dependencies
Use [stack/scripts/install_ubuntu_dependencies.sh](stack/scripts/install_ubuntu_dependencies.sh) to install all packages needed to run & build BTC-HyperChat.com.

## Setting up the database & Keycloak instance configuration
Use [stack/scripts/setup_fresh_application_data.sh](stack/scripts/setup_fresh_application_data.sh) to configure a PostgreSQL docker volume that contains all data needed to start both the backend application & the Keycloak instance

## Setting up the nginx server
Use [stack/scripts/setup_fresh_front_facing_nginx.sh](stack/scripts/setup_fresh_front_facing_nginx.sh) to setup nginx with let's encrypt.

## Starting the application
Use [stack/scripts/run_application.sh](stack/scripts/run_application.sh) to build all Docker images and start the application.
