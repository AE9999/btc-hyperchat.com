package com.ae.btc.chat.registration.event.listener

import org.jboss.logging.Logger
import org.keycloak.Config;
import org.keycloak.events.EventListenerProvider;
import org.keycloak.events.EventListenerProviderFactory;
import org.keycloak.models.KeycloakSession;
import org.keycloak.models.KeycloakSessionFactory;


// https://aboutbits.it/blog/2020-11-23-keycloak-custom-event-listener

class RegistrationEventListenerProviderFactory : EventListenerProviderFactory  {

    private val log: Logger = Logger.getLogger(RegistrationEventListenerProviderFactory::class.java)

    override fun create(keycloakSession: KeycloakSession?): EventListenerProvider? {
        return keycloakSession?. let {
            RegistrationEventListener(it)
        }
    }

    override fun init(config: Config.Scope?) {}

    override fun postInit(factory: KeycloakSessionFactory?) {}

    override fun close() {}

    override fun getId(): String {
        return "registration-event-listener"
    }

}
