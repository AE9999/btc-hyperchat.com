// This file can be replaced during build by using the `fileReplacements` array.
// `ng build` replaces `environment.ts` with `environment.prod.ts`.
// The list of file replacements can be found in `angular.json`.

export const environment = {
  production: false,
  testnet: true,
  btcpayServer: "https://testnet.demo.btcpayserver.org",
  api: "http://<YOUR-BACKEND-LOCATION>:8081/api",
  resources: "/images",
  keycloakUrl: "http://<YOUR-BACKEND-LOCATION>:8080",
  keycloakRealm: 'btc_super_chat',
  keycloakClient: 'frontend',
  myReceivedBtcChatStreamUrl: 'ws://<YOUR-BACKEND-LOCATION>:8081/ws/btcchat/myreceivedbtcchatstream',
  logoutUrl: 'https://<YOUR-BACKEND-LOCATION>',
  registerBtcChatComponent: {
    minNameLength:2,
    maxNameLength: 25,
    maxMessageLength: 200
  },
  searchProfileAutocompleteComponent: {
    minLengthTerm: 3,
    debounceTime: 1000
  }
};

/*
 * For easier debugging in development mode, you can import the following file
 * to ignore zone related error stack frames such as `zone.run`, `zoneDelegate.invokeTask`.
 *
 * This import should be commented out in production mode because it will have a negative impact
 * on performance if an error is thrown.
 */
// import 'zone.js/plugins/zone-error';  // Included with Angular CLI.
