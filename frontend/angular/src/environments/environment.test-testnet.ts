export const environment = {
  production: false,
  testnet: true,
  btcpayServer: "https://testnet.demo.btcpayserver.org",
  api: "http://localhost:8081/api",
  resources: "/images",
  keycloakUrl: "http://localhost:8080",
  keycloakRealm: 'btc_chat',
  keycloakClient: 'frontend',
  myReceivedBtcChatStreamUrl: 'ws:///localhost:8081/ws/btcchat/myreceivedbtcchatstream',
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
