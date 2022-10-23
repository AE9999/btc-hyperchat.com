export const environment = {
  production: true,
  testnet: false,
  btcpayServer: "https://<YOUR-BTC-PAY-SERVER>",
  api: "http://<YOUR-BACKEND-LOCATION>:8081/api",
  resources: "/images",
  keycloakUrl: "http://<YOUR-BACKEND-LOCATION>:8080",
  keycloakRealm: 'btc_hyper_chat',
  keycloakClient: 'frontend',
  myReceivedBtcChatStreamUrl: 'ws://<YOUR-BACKEND-LOCATION>:8081/ws/btcchat/myreceivedbtcchatstream',
  logoutUrl: 'http://localhost:4200',
  registerBtcChatComponent: {
    minNameLength:2,
    maxNameLength: 25,
    maxMessageLength: 200
  },
  searchProfileAutocompleteComponent: {
    minLengthTerm: 1,
    debounceTime: 1000
  }
};
