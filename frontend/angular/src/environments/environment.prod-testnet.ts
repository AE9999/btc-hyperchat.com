export const environment = {
  production: true,
  testnet: false,
  btcpayServer: "https://testnet.demo.btcpayserver.org",
  api: "/api",
  resources: "/images",
  keycloakUrl: "/auth",
  keycloakRealm: 'btc_super_chat',
  keycloakClient: 'frontend',
  myReceivedBtcChatStreamUrl: 'wss://<YOUR-DOMAIN>/ws/btcchat/myreceivedbtcchatstream',
  logoutUrl: 'https://<YOUR-DOMAIN>',
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
