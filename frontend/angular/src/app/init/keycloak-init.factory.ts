import { KeycloakService } from "keycloak-angular";
import { environment } from './../../environments/environment';


export function initializeKeycloak(
  keycloak: KeycloakService
) {
  return () =>
    keycloak.init({
      config: {
        url: environment.keycloakUrl ,
        realm: environment.keycloakRealm,
        clientId: environment.keycloakClient,
      },
      initOptions: {
        onLoad: 'check-sso',
        silentCheckSsoRedirectUri:
          window.location.origin + '/assets/silent-check-sso.html'
      }
    });
}
