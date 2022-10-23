package com.ae.btc.chat.registration.event.listener.model


// keycloak_id
data class CreateUserRequest (
    val keycloakId: String,
    val email: String,
    val username: String,
)
