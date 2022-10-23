plugins {
    id("org.jetbrains.kotlin.jvm") version "1.5.30"
}

apply(plugin = "org.jetbrains.kotlin.jvm")

repositories {
    mavenLocal()
    mavenCentral()
    jcenter()
    maven {
        url = uri("https://plugins.gradle.org/m2/")
    }
}

java {
    sourceCompatibility = JavaVersion.VERSION_11
    targetCompatibility = JavaVersion.VERSION_11
}

tasks.withType<org.jetbrains.kotlin.gradle.tasks.KotlinCompile>().configureEach {
    kotlinOptions {
        jvmTarget = "11"
    }
}

// Due to https://stackoverflow.com/questions/44197521/gradle-project-java-lang-noclassdeffounderror-kotlin-jvm-internal-intrinsics
tasks.withType<Jar> {
    duplicatesStrategy = DuplicatesStrategy.EXCLUDE

    // Otherwise you'll get a "No main manifest attribute" error
//    manifest {
//        attributes["Main-Class"] = "com.example.MainKt"
//    }

    // To add all of the dependencies
    from(sourceSets.main.get().output)

    dependsOn(configurations.runtimeClasspath)
    from({
        configurations.runtimeClasspath.get().filter { it.name.endsWith("jar") }.map { zipTree(it) }
    })
}

dependencies {
    val kotlinVersion: String by project
    val keycloakVersion: String by project
    val gsonVersion: String by project
    val httpClientVersion: String by project

    // implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:$kotlinxCoRoutines") kotlinxCoRoutines
    implementation("org.jetbrains.kotlin:kotlin-stdlib:$kotlinVersion")
    implementation("org.jetbrains.kotlin:kotlin-reflect:$kotlinVersion")
    implementation("org.jetbrains.kotlin:kotlin-stdlib-jdk8:$kotlinVersion")

    compileOnly("org.keycloak:keycloak-server-spi:$keycloakVersion")
    compileOnly("org.keycloak:keycloak-server-spi-private:$keycloakVersion")
    compileOnly("org.keycloak:keycloak-services:$keycloakVersion")

    implementation("com.google.code.gson:gson:$gsonVersion")
    implementation("org.apache.httpcomponents:httpclient:$httpClientVersion")
}
