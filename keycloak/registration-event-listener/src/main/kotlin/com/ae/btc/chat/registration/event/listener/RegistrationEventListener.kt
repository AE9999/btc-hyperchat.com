package com.ae.btc.chat.registration.event.listener

import com.ae.btc.chat.registration.event.listener.model.CreateUserRequest
import com.google.gson.Gson
import org.apache.http.HttpStatus
import org.apache.http.client.methods.HttpPost
import org.apache.http.entity.StringEntity
import org.apache.http.impl.client.HttpClientBuilder
import org.jboss.logging.Logger
import org.keycloak.events.Event
import org.keycloak.events.EventListenerProvider
import org.keycloak.events.EventType
import org.keycloak.events.admin.AdminEvent
import org.keycloak.models.KeycloakSession
import org.keycloak.models.RealmProvider
import java.nio.charset.StandardCharsets
import java.util.*


class RegistrationEventListener : EventListenerProvider {

    enum class LogEvents {
        RECIEVED_KEYCLOACK_REGISTRATION_EVENT,
        SEND_REGISTRATION_EVENT_TO_BTC_CHAT_BACKEND,
        SEND_REGISTRATION_EVENT_TO_BTC_CHAT_SUCCESS_ANSWER_RECEIVED,
        SEND_REGISTRATION_EVENT_TO_BTC_CHAT_SUCCESS,
        SEND_REGISTRATION_EVENT_TO_BTC_CHAT_FAILURE,
    }

    private val log: Logger = Logger.getLogger(RegistrationEventListener::class.java)

    private var session: KeycloakSession? = null

    private var model: RealmProvider? = null

    constructor(session: KeycloakSession) {
        this.session = session
        this.model = session.realms()
    }

    // Todo Make async
    override fun onEvent(event: Event ?) {

        if (EventType.REGISTER == event?.type) {
            log.infof("%s", LogEvents.RECIEVED_KEYCLOACK_REGISTRATION_EVENT)
            val realm = model?.getRealm(event.realmId)

            val newRegisteredUser = session!!.users().getUserById(event.userId, realm)

            val endpoint = System.getenv("KEYCLOAK_BTC_CHAT_URL");

            val gson = Gson()

            val client = HttpClientBuilder.create().build()

            var httpPost = HttpPost(endpoint)

            val request = CreateUserRequest(username= newRegisteredUser.username,
                                            email=newRegisteredUser.email,
                                            keycloakId = event.userId)

            log.infof(LogEvents.RECIEVED_KEYCLOACK_REGISTRATION_EVENT.toString())

            httpPost.entity = StringEntity(gson.toJson(request))
            httpPost.setHeader("Accept", "application/json")
            httpPost.setHeader("Content-type", "application/json")

            // Setting auth stuff as per https://stackoverflow.com/questions/20914311/httpclientbuilder-basic-auth
            val credentials = String.format(
                    "%s:%s",
                    System.getenv("KEYCLOAK_BTC_CHAT_REGISTER_USER_USERNAME"),
                    System.getenv("KEYCLOAK_BTC_CHAT_REGISTER_USER_PASSWORD"),
            )
            val encodedCredentials = Base64.getEncoder().encode(credentials.toByteArray(StandardCharsets.UTF_8))
            httpPost.setHeader("Authorization", "Basic " + String(encodedCredentials, StandardCharsets.UTF_8))



            log.infof("%s, %s, %s %s",
                       LogEvents.SEND_REGISTRATION_EVENT_TO_BTC_CHAT_BACKEND, httpPost,
                       StringEntity(gson.toJson(request)),
                      credentials)

            try {
                val response = client.execute(httpPost)

                log.infof("%s, %s",
                        LogEvents.SEND_REGISTRATION_EVENT_TO_BTC_CHAT_SUCCESS_ANSWER_RECEIVED,
                        response)

                log.info(if (response.statusLine.statusCode != HttpStatus.SC_OK) {
                    LogEvents.SEND_REGISTRATION_EVENT_TO_BTC_CHAT_FAILURE
                } else {
                    LogEvents.SEND_REGISTRATION_EVENT_TO_BTC_CHAT_SUCCESS
                })

            } catch (e: Exception) {
                log.infof("%s, %s",
                        LogEvents.SEND_REGISTRATION_EVENT_TO_BTC_CHAT_FAILURE,
                        e)
            }

            client.close()

        }
    }

    override fun onEvent(adminEvent: AdminEvent?, b: Boolean) {}

    override fun close() {}
}
