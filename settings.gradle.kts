pluginManagement {
    repositories {
        mavenLocal()
        mavenCentral()
        jcenter()
        maven {
            url = uri("https://plugins.gradle.org/m2/")
        }
    }
}

buildscript {
    repositories {
        mavenLocal()
        mavenCentral()
        jcenter()
        maven {
            url = uri("https://plugins.gradle.org/m2/")
        }
    }
    dependencies {
        classpath("gradle.plugin.net.vivin:gradle-semantic-build-versioning:4.0.0")
    }
}

include(":backend",
        ":frontend",
        ":keycloak:project-build",
        ":keycloak:registration-event-listener"

)

apply(plugin = "net.vivin.gradle-semantic-build-versioning")
rootProject.name = "btc-chat"
